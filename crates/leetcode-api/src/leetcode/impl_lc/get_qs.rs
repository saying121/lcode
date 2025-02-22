use std::sync::atomic::Ordering;

use futures::{StreamExt, stream};
use lcode_config::global::G_USER_CONFIG;
use miette::Result;
use tracing::{debug, error};

use crate::{
    Json,
    dao::{InsertToDB, query::Query, save_info::FileInfo},
    entities::index,
    leetcode::{
        CATEGORIES, CUR_QS_INDEX_NUM, CUR_TOPIC_QS_INDEX_NUM, IdSlug, LeetCode, TOTAL_QS_INDEX_NUM,
        TOTAL_TOPIC_QS_INDEX_NUM,
        graphqls::GraphqlQuery,
        question::{
            pb_list::PbListData,
            qs_detail::{Question, QuestionData},
            qs_index::Problems,
        },
    },
};

impl LeetCode {
    /// get leetcode index
    ///
    /// # Errors
    ///
    /// - network error
    /// - leetcode url change
    /// - `DbErr`
    /// * `force`: when true will force update
    pub async fn sync_problem_index(&self) -> Result<()> {
        stream::iter(CATEGORIES)
            .for_each_concurrent(None, |category| async move {
                let all_pb_url = G_USER_CONFIG
                    .urls
                    .mod_all_pb_api(category);

                // try 6 times
                let mut count = 0;
                let pbs: Problems = loop {
                    match self
                        .request(&all_pb_url, None, self.headers.clone())
                        .await
                    {
                        Ok(v) => break v,
                        Err(err) => {
                            count += 1;
                            error!("{}, frequency: {}", err, count);
                            if count > 5 {
                                break Problems::default();
                            }
                        },
                    }
                };

                TOTAL_QS_INDEX_NUM.fetch_add(pbs.num_total, Ordering::Relaxed);

                stream::iter(pbs.stat_status_pairs)
                    .for_each_concurrent(None, |mut problem| async move {
                        problem
                            .insert_to_db(category.to_owned())
                            .await;
                        CUR_QS_INDEX_NUM.fetch_add(1, Ordering::Relaxed);
                    })
                    .await;
            })
            .await;

        TOTAL_QS_INDEX_NUM.store(0, Ordering::Relaxed);
        CUR_QS_INDEX_NUM.store(0, Ordering::Relaxed);
        Ok(())
    }

    /// get question titleSlug and topicTags info
    pub async fn sync_index_topic(&self) -> Result<()> {
        let url = &G_USER_CONFIG.urls.graphql;

        let graphql = GraphqlQuery::get_count();
        let data: PbListData = self
            .request(url, Some(&graphql.0), self.headers.clone())
            .await?;
        let total = data.data.problemset_question_list.total;

        stream::iter((0..total).step_by(100))
            .for_each_concurrent(None, |skip| async move {
                let graphql = GraphqlQuery::new(skip);

                // try 4 times
                let mut count = 0;
                let data: PbListData = loop {
                    match self
                        .request(url, Some(&graphql), self.headers.clone())
                        .await
                    {
                        Ok(it) => break it,
                        Err(err) => {
                            count += 1;
                            error!("{}, frequency: {}", err, count);
                            if count > 3 {
                                break PbListData::default();
                            }
                        },
                    }
                };

                TOTAL_TOPIC_QS_INDEX_NUM.fetch_add(100, Ordering::Relaxed);

                let pb_list = data
                    .data
                    .problemset_question_list
                    .questions;

                stream::iter(pb_list)
                    .for_each_concurrent(None, |mut new_pb| async move {
                        new_pb.insert_to_db(0).await;
                        CUR_TOPIC_QS_INDEX_NUM.fetch_add(1, Ordering::Relaxed);
                    })
                    .await;
            })
            .await;

        TOTAL_TOPIC_QS_INDEX_NUM.store(0, Ordering::Relaxed);
        CUR_TOPIC_QS_INDEX_NUM.store(0, Ordering::Relaxed);
        Ok(())
    }

    async fn get_qs_detail_helper_force(&self, pb: &index::Model) -> Result<Question> {
        let json: Json = GraphqlQuery::qs_detail(&pb.question_title_slug);

        let mut qs: QuestionData = self
            .request(
                &G_USER_CONFIG.urls.graphql,
                Some(&json),
                self.headers.clone(),
            )
            .await?;

        qs.data.question.qs_slug = Some(pb.question_title_slug.clone());
        qs.data
            .question
            .insert_one(pb.question_id)
            .await;

        Ok(qs.data.question)
    }

    /// Get the details of the problem, and if it's in the cache, use it.
    /// Write data to file.
    ///
    /// * `id`: id of the problem
    /// * `force`: when true, the cache will be re-fetched
    /// * `write`: when true, write to file
    pub async fn get_qs_detail(
        &self,
        idslug: IdSlug,
        force: bool,
        write: bool,
    ) -> Result<Question> {
        if let IdSlug::Id(id) = idslug {
            if id == 0 {
                miette::bail!("Question Id require large 0")
            }
        }
        let pb = Query::get_question_index(&idslug).await?;
        debug!("pb: {:?}", pb);
        let detail = if force {
            self.get_qs_detail_helper_force(&pb)
                .await?
        }
        else {
            let temp = Query::query_detail_by_id(pb.question_id).await?;

            let the_detail = temp.unwrap_or_default();
            let detail: Question = serde_json::from_str(&the_detail.content).unwrap_or_default();
            // deserialize failed
            if detail.qs_slug.is_none() {
                self.get_qs_detail_helper_force(&pb)
                    .await?
            }
            else {
                detail
            }
        };

        if write {
            let chf = FileInfo::build(&pb).await?;
            chf.write_to_file(&detail).await?;
        }

        Ok(detail)
    }
}
