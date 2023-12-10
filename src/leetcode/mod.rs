mod graphqls;
pub mod pb_list;
pub mod qs_detail;
pub mod qs_index;
pub mod resps;

use std::{
    collections::HashMap,
    fmt::Display,
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};

use futures::StreamExt;
use miette::{IntoDiagnostic, Result};
use regex::Regex;
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use sea_orm::EntityTrait;
use serde_json::Value;
use tokio::{join, time::sleep};
use tracing::{debug, error, info, instrument, trace};

use self::{
    graphqls::*,
    leetcode_send::*,
    qs_detail::*,
    qs_index::Problems,
    resps::{run_res::RunResult, submit_list::SubmissionList, *},
};
use crate::{
    config::{
        global::{glob_user_config, CATEGORIES},
        Headers,
    },
    dao::{get_question_index_exact, glob_db, save_info::CacheFile, InsertToDB},
    entities::{prelude::*, *},
    Json,
};

pub static TOTAL_QS_INDEX_NUM: AtomicU32 = AtomicU32::new(0);
pub static CUR_QS_INDEX_NUM: AtomicU32 = AtomicU32::new(0);
pub static TOTAL_NEW_QS_INDEX_NUM: AtomicU32 = AtomicU32::new(0);
pub static CUR_NEW_QS_INDEX_NUM: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone)]
pub enum IdSlug {
    Id(u32),
    Slug(String),
}

impl Display for IdSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(num) => num.fmt(f),
            Self::Slug(slug) => slug.fmt(f),
        }
    }
}

/// interact with leetcode.com/cn
#[derive(Debug, Default)]
pub struct LeetCode {
    pub client: Client,
    pub headers: HeaderMap,
}

impl LeetCode {
    /// Create a `LeetCode` instance and initialize some variables
    pub async fn new() -> Result<Self> {
        let client = ClientBuilder::new()
            .gzip(true)
            .connect_timeout(Duration::from_secs(30))
            .build()
            .into_diagnostic()?;

        Ok(Self {
            client,
            headers: Headers::new().await?.headers,
        })
    }

    /// get leetcode index
    ///
    /// # Errors
    ///
    /// - network error
    /// - leetcode url change
    /// - DbErr
    /// * `force`: when true will force update
    #[instrument(skip(self))]
    pub async fn sync_problem_index(&self) -> Result<()> {
        futures::stream::iter(CATEGORIES)
            .for_each_concurrent(None, |category| async move {
                let all_pb_url = glob_user_config().mod_all_pb_api(category);

                // try 6 times
                let mut count = 0;
                let resp_json = loop {
                    match fetch(
                        &self.client,
                        &all_pb_url,
                        None,
                        SendMode::Get,
                        self.headers.clone(),
                    )
                    .await
                    {
                        Ok(v) => break v,
                        Err(err) => {
                            count += 1;
                            error!("{}, frequency: {}", err, count);
                            if count > 5 {
                                break Value::default();
                            }
                        }
                    }
                };
                let pbs: Problems = serde_json::from_value(resp_json).unwrap_or_default();

                TOTAL_QS_INDEX_NUM.fetch_add(pbs.num_total, Ordering::Release);

                futures::stream::iter(pbs.stat_status_pairs)
                    .for_each_concurrent(None, |mut problem| async move {
                        problem
                            .insert_to_db(category.to_owned())
                            .await;
                        CUR_QS_INDEX_NUM.fetch_add(1, Ordering::Release);
                    })
                    .await;
            })
            .await;

        TOTAL_QS_INDEX_NUM.store(0, Ordering::Release);
        CUR_QS_INDEX_NUM.store(0, Ordering::Release);
        Ok(())
    }

    /// get question titleSlug and topicTags info
    pub async fn new_sync_index(&self) -> Result<()> {
        let url = &glob_user_config().urls.graphql;

        let graphql = QueryProblemSet::get_count();
        let resp_json = fetch(
            &self.client,
            url,
            Some(graphql.json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await?;
        let data: pb_list::Data = serde_json::from_value(resp_json).unwrap_or_default();
        let total = data
            .data
            .problemset_question_list
            .total;

        futures::stream::iter((0..total).step_by(100))
            .for_each_concurrent(None, |skip| async move {
                let graphql = QueryProblemSet::new(skip);

                // try 3 times
                let mut count = 0;
                let resp_json = loop {
                    match fetch(
                        &self.client,
                        url,
                        Some(graphql.json.clone()),
                        SendMode::Post,
                        self.headers.clone(),
                    )
                    .await
                    {
                        Ok(it) => break it,
                        Err(err) => {
                            count += 1;
                            error!("{}, frequency: {}", err, count);
                            if count > 2 {
                                break Value::default();
                            }
                        }
                    }
                };

                TOTAL_NEW_QS_INDEX_NUM.fetch_add(100, Ordering::Release);

                let data: pb_list::Data =
                    serde_json::from_value(resp_json).unwrap_or_default();
                let pb_list = data
                    .data
                    .problemset_question_list
                    .questions;

                futures::stream::iter(pb_list)
                    .for_each_concurrent(None, |mut new_pb| async move {
                        new_pb.insert_to_db(0).await;
                    })
                    .await;
                CUR_NEW_QS_INDEX_NUM.fetch_add(1, Ordering::Release);
            })
            .await;

        TOTAL_NEW_QS_INDEX_NUM.store(0, Ordering::Release);
        CUR_NEW_QS_INDEX_NUM.store(0, Ordering::Release);
        Ok(())
    }

    async fn get_qs_detail_helper_force(&self, pb: &index::Model) -> Result<Question> {
        let json: Json = init_qs_detail_grql(&pb.question_title_slug);

        let pb_json = fetch(
            &self.client,
            &glob_user_config().urls.graphql,
            Some(json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await?;

        let pb_data = pb_json
            .get("data")
            .cloned()
            .unwrap_or_default()
            .get("question")
            .cloned()
            .unwrap_or_default();

        debug!("the get detail json: {}", pb_data);

        let qs = Question::from_serde(pb_data, pb.question_title_slug.clone())?;

        qs.insert_one(pb.question_id).await;

        Ok(qs)
    }

    /// Get the details of the problem, and if it's in the cache, use it.
    /// Write data to file.
    ///
    /// * `id`: id of the problem
    /// * `force`: when true, the cache will be re-fetched
    #[instrument(skip(self))]
    pub async fn get_qs_detail(&self, idslug: IdSlug, force: bool) -> Result<Question> {
        if let IdSlug::Id(id) = idslug {
            if id == 0 {
                return Ok(Question::default());
            }
        }

        let pb = get_question_index_exact(&idslug).await?;

        debug!("pb: {:?}", pb);

        let mut detail;

        if force {
            detail = self
                .get_qs_detail_helper_force(&pb)
                .await?;
        } else {
            let temp = Detail::find_by_id(pb.question_id)
                .one(glob_db().await)
                .await
                .into_diagnostic()?;

            let the_detail = temp.unwrap_or_default();
            detail = serde_json::from_str(&the_detail.content).unwrap_or_default();
            if detail.qs_slug.is_none() {
                detail = self
                    .get_qs_detail_helper_force(&pb)
                    .await?;
            }
        }

        let chf = CacheFile::new(&idslug).await?;
        chf.write_to_file(detail.clone())
            .await?;

        Ok(detail)
    }

    /// submit code by id or slug, once submit one question
    ///
    /// * `idslug`: id or slug
    pub async fn submit_code(&self, idslug: IdSlug) -> Result<(SubmitInfo, RunResult)> {
        let (code, pb) = join!(
            self.get_user_code(idslug.clone()),
            get_question_index_exact(&idslug)
        );
        let ((code, _test_case), pb) = (code?, pb?);

        let mut json: Json = HashMap::new();
        json.insert("lang", glob_user_config().config.lang.clone());
        json.insert("question_id", pb.question_id.to_string());
        json.insert("typed_code", code);

        trace!("submit insert json: {:#?}", json);

        let resp_json = fetch(
            &self.client,
            &glob_user_config().mod_submit(&pb.question_title_slug),
            Some(json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await?;

        debug!("submit resp_json: {:?}", resp_json);

        let sub_id: SubmitInfo = match serde_json::from_value(resp_json) {
            Ok(it) => it,
            Err(err) => {
                return Ok((
                    SubmitInfo::default(),
                    RunResult {
                        status_msg: err.to_string(),
                        ..Default::default()
                    },
                ))
            }
        };
        trace!("out submit id: {}", sub_id.submission_id);

        let last_sub_result = self
            .get_one_submit_res(&sub_id)
            .await?;
        debug!("last submit result: {:#?}", last_sub_result);

        Ok((sub_id, last_sub_result))
    }

    /// Get one submit info
    ///
    /// * `sub_id`: be fetch submission_id
    #[instrument(skip(self))]
    pub async fn get_one_submit_res(&self, sub_id: &SubmitInfo) -> Result<RunResult> {
        let test_res_url =
            glob_user_config().mod_submissions(&sub_id.submission_id.to_string());
        trace!("start get last submit detail");

        let mut count = 0;
        loop {
            sleep(Duration::from_millis(700)).await;

            let resp_json = fetch(
                &self.client,
                &test_res_url,
                None,
                SendMode::Get,
                self.headers.clone(),
            )
            .await?;

            debug!("this detail json: {:#?}", resp_json);

            match serde_json::from_value::<RunResult>(resp_json) {
                Ok(v) => {
                    debug!("the submit resp: {:#?}", v);
                    if v.state == "SUCCESS" {
                        return Ok(v);
                    }
                }
                Err(err) => {
                    error!("{:?}", err);
                    info!("waiting resp");
                }
            }

            if count > 9 {
                return Ok(RunResult {
                    status_msg: "Get the submit result error, please check your code, \
                                   it may fail to execute, or check your network"
                        .to_owned(),
                    ..Default::default()
                });
            }
            count += 1;
        }
    }

    /// Get all submission results for a question
    #[instrument(skip(self))]
    pub async fn all_submit_res(&self, idslug: IdSlug) -> Result<SubmissionList> {
        let pb = get_question_index_exact(&idslug).await?;

        let json: Json = init_subit_list_grql(&pb.question_title_slug);

        let resp_json = fetch(
            &self.client,
            &glob_user_config().urls.graphql,
            Some(json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await?;

        let be_serde = resp_json
            .get("data")
            .cloned()
            .unwrap_or_default()
            .get("submissionList")
            .cloned()
            .unwrap_or_default();
        debug!("be serde submission list: {:#?}", be_serde);

        let sub_detail: submit_list::SubmissionList =
            serde_json::from_value(be_serde).into_diagnostic()?;

        trace!("all submit detail: {:#?}", sub_detail);

        Ok(sub_detail)
    }

    #[instrument(skip(self))]
    pub async fn test_code(&self, idslug: IdSlug) -> Result<(TestInfo, RunResult)> {
        let (code, pb) = join!(
            self.get_user_code(idslug.clone()),
            get_question_index_exact(&idslug)
        );
        let ((code, test_case), pb) = (code?, pb?);
        debug!("code:\n{}", code);

        let mut json: Json = HashMap::new();
        json.insert("lang", glob_user_config().config.lang.clone());
        json.insert("question_id", pb.question_id.to_string());
        json.insert("typed_code", code);
        json.insert("data_input", test_case);

        let resp_json = match fetch(
            &self.client,
            &glob_user_config().mod_test(&pb.question_title_slug),
            Some(json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await
        {
            Ok(it) => it,
            Err(err) => {
                return Ok((
                    TestInfo::default(),
                    RunResult {
                        status_msg: err.to_string(),
                        ..Default::default()
                    },
                ));
            }
        };

        debug!("test resp json: {:#?}", resp_json);

        let test_info: TestInfo = serde_json::from_value(resp_json).into_diagnostic()?;
        debug!("test info: {:#?}", test_info);

        let test_result = self
            .get_test_res(&test_info)
            .await?;

        Ok((test_info, test_result))
    }

    /// Get the last submission results for a question
    async fn get_test_res(&self, test_info: &TestInfo) -> Result<RunResult> {
        let mut count = 0;
        loop {
            sleep(Duration::from_millis(700)).await;

            let resp_json = fetch(
                &self.client.clone(),
                &glob_user_config().mod_submissions(&test_info.interpret_id),
                None,
                SendMode::Get,
                self.headers.clone(),
            )
            .await?;

            debug!("test resp json: {:#?}", resp_json);

            match serde_json::from_value::<RunResult>(resp_json.clone()) {
                Ok(v) => {
                    debug!("the test detail res: {:#?}", v);
                    if v.state == "SUCCESS" {
                        return Ok(v);
                    }
                }
                Err(err) => {
                    error!("{:?}", err);
                    info!("waiting resp");
                }
            }

            if count > 9 {
                return Ok(RunResult {
                    status_msg: "Get the test result error, please check your network,\
                    or check test case it may not correct"
                        .to_owned(),
                    ..Default::default()
                });
            }
            count += 1;
        }
    }

    /// Get user code as string,(`code`, `test case`)
    pub async fn get_user_code(&self, idslug: IdSlug) -> Result<(String, String)> {
        let chf = CacheFile::new(&idslug).await?;
        let (code, mut test_case) = chf.get_user_code(&idslug).await?;

        if test_case.is_empty() {
            test_case = self
                .get_qs_detail(idslug, false)
                .await?
                .example_testcases;
        }
        let (start, end, _, _) = glob_user_config().get_lang_info();
        let code_re =
            Regex::new(&format!(r"(?s){}\n(?P<code>.*){}", start, end)).unwrap();

        // sep code just get needed
        let res = match code_re.captures(&code) {
            Some(val) => val["code"].to_owned(),
            None => code,
        };

        Ok((res, test_case))
    }
}

mod leetcode_send {
    use miette::{miette, IntoDiagnostic, Result};
    use reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    };
    use serde_json::Value;
    use tracing::trace;

    use crate::config::Headers;

    use super::Json;

    pub(super) enum SendMode {
        Get,
        Post,
    }

    pub(super) async fn fetch(
        client: &Client,
        url: &str,
        json: Option<Json>,
        mode: SendMode,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<Value> {
        let headers = Headers::mod_headers(headers, vec![("Referer", url)])?;

        let temp = match mode {
            SendMode::Get => client.get(url),
            SendMode::Post => client.post(url).json(&json),
        };

        let resp = temp
            .headers(headers)
            .send()
            .await
            .into_diagnostic()?;
        trace!("respond: {:#?}", resp);

        resp.json().await.map_err(|e| {
            miette!(
                "Error: {}, check your cookies(Confirm you are logged in) or network.",
                e
            )
        })
    }
}
