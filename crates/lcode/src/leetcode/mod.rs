mod graphqls;
mod headers;
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
use lcode_config::config::global::USER_CONFIG;
use miette::{IntoDiagnostic, Result};
use regex::Regex;
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use tokio::{join, time::sleep};
use tracing::{debug, error, instrument, trace};

use self::{
    graphqls::*,
    headers::Headers,
    leetcode_send::*,
    pb_list::PbListData,
    qs_detail::*,
    qs_index::Problems,
    resps::{run_res::RunResult, submit_list::SubmissionList, user_data::UserStatus, *},
};
use crate::{
    dao::{get_question_index, query_detail_by_id, save_info::CacheFile, InsertToDB},
    entities::*,
    leetcode::resps::{checkin::CheckInData, user_data::GlobData},
    Json,
};

pub const CATEGORIES: [&str; 8] = [
    "algorithms",
    "concurrency",
    "database",
    "javascript",
    "lcci",
    "lcof",
    "pandas",
    "shell",
];

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
    pub client:  Client,
    pub headers: HeaderMap,
}

impl LeetCode {
    /// Create a `LeetCode` instance and initialize some variables
    pub async fn build() -> Result<Self> {
        let client = ClientBuilder::new()
            .gzip(true)
            .connect_timeout(Duration::from_secs(30))
            .build()
            .into_diagnostic()?;

        Ok(Self {
            client,
            headers: Headers::build_default()
                .await?
                .headers,
        })
    }
    pub async fn get_user_info(&self) -> Result<UserStatus> {
        let json = global_data();

        let resp: GlobData = fetch(
            &self.client,
            &USER_CONFIG.urls.graphql,
            Some(json.clone()),
            SendMode::Post,
            self.headers.clone(),
        )
        .await
        .unwrap_or_default();

        Ok(resp.data.user_status)
    }
    /// return order (cn, com)
    pub async fn daily_checkin(&self) -> Result<(CheckInData, CheckInData)> {
        let json: Json = daily_checkin_grql();

        let header_cn = Headers::build("leetcode.cn")
            .await
            .unwrap_or_default();
        let header_com = Headers::build("leetcode.com")
            .await
            .unwrap_or_default();

        let resp_cn = fetch::<CheckInData>(
            &self.client,
            "https://leetcode.cn/graphql",
            Some(json.clone()),
            SendMode::Post,
            header_cn.headers,
        );

        let resp_com = fetch::<CheckInData>(
            &self.client,
            "https://leetcode.com/graphql",
            Some(json),
            SendMode::Post,
            header_com.headers,
        );
        let (resp_cn, resp_com) = join!(resp_cn, resp_com);
        let cn_data = resp_cn?;
        let com_data = resp_com?;

        Ok((cn_data, com_data))
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
                let all_pb_url = USER_CONFIG.mod_all_pb_api(category);

                // try 6 times
                let mut count = 0;
                let pbs: Problems = loop {
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
                                break Problems::default();
                            }
                        },
                    }
                };

                TOTAL_QS_INDEX_NUM.fetch_add(pbs.num_total, Ordering::Relaxed);

                futures::stream::iter(pbs.stat_status_pairs)
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
    pub async fn new_sync_index(&self) -> Result<()> {
        let url = &USER_CONFIG.urls.graphql;

        let graphql = QueryProblemSet::get_count();
        let data: PbListData = fetch(
            &self.client,
            url,
            Some(graphql.json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await?;
        let total = data
            .data
            .problemset_question_list
            .total;

        futures::stream::iter((0..total).step_by(100))
            .for_each_concurrent(None, |skip| async move {
                let graphql = QueryProblemSet::new(skip);

                // try 3 times
                let mut count = 0;
                let data: PbListData = loop {
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
                                break PbListData::default();
                            }
                        },
                    }
                };

                TOTAL_NEW_QS_INDEX_NUM.fetch_add(100, Ordering::Relaxed);

                let pb_list = data
                    .data
                    .problemset_question_list
                    .questions;

                futures::stream::iter(pb_list)
                    .for_each_concurrent(None, |mut new_pb| async move {
                        new_pb.insert_to_db(0).await;
                        CUR_NEW_QS_INDEX_NUM.fetch_add(1, Ordering::Relaxed);
                    })
                    .await;
            })
            .await;

        TOTAL_NEW_QS_INDEX_NUM.store(0, Ordering::Relaxed);
        CUR_NEW_QS_INDEX_NUM.store(0, Ordering::Relaxed);
        Ok(())
    }

    async fn get_qs_detail_helper_force(&self, pb: &index::Model) -> Result<Question> {
        let json: Json = init_qs_detail_grql(&pb.question_title_slug);

        let mut qs: QuestionData = fetch(
            &self.client,
            &USER_CONFIG.urls.graphql,
            Some(json),
            SendMode::Post,
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
    #[instrument(skip(self))]
    pub async fn get_qs_detail(&self, idslug: IdSlug, force: bool) -> Result<Question> {
        if let IdSlug::Id(id) = idslug {
            if id == 0 {
                return Ok(Question::default());
            }
        }

        let pb = get_question_index(&idslug).await?;

        debug!("pb: {:?}", pb);

        let detail = if force {
            self.get_qs_detail_helper_force(&pb)
                .await?
        }
        else {
            let temp = query_detail_by_id(pb.question_id).await?;

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

        let chf = CacheFile::build(&pb).await?;
        chf.write_to_file(&detail).await?;

        Ok(detail)
    }

    /// submit code by id or slug, once submit one question
    ///
    /// * `idslug`: id or slug
    pub async fn submit_code(&self, idslug: IdSlug) -> Result<(SubmitInfo, RunResult)> {
        let (code, pb) = join!(
            self.get_user_code(idslug.clone()),
            get_question_index(&idslug)
        );
        let ((code, _test_case), pb) = (code?, pb?);

        let mut json: Json = HashMap::new();
        json.insert("lang", USER_CONFIG.config.lang.clone());
        json.insert("question_id", pb.question_id.to_string());
        json.insert("typed_code", code);

        trace!("submit insert json: {:#?}", json);

        let sub_info: SubmitInfo = fetch(
            &self.client,
            &USER_CONFIG.mod_submit(&pb.question_title_slug),
            Some(json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await?;

        let last_sub_result = self
            .get_one_submit_res(&sub_info)
            .await?;
        debug!("last submit result: {:#?}", last_sub_result);

        Ok((sub_info, last_sub_result))
    }

    /// Get one submit info
    ///
    /// * `sub_id`: be fetch submission_id
    #[instrument(skip(self))]
    pub async fn get_one_submit_res(&self, sub_id: &SubmitInfo) -> Result<RunResult> {
        let test_res_url = USER_CONFIG.mod_submissions(&sub_id.submission_id.to_string());
        trace!("start get last submit detail");

        let mut count = 0;
        loop {
            sleep(Duration::from_millis(700)).await;

            let resp_json: RunResult = fetch(
                &self.client,
                &test_res_url,
                None,
                SendMode::Get,
                self.headers.clone(),
            )
            .await?;
            if resp_json.state == "SUCCESS" {
                return Ok(resp_json);
            }

            if count > 9 {
                return Ok(RunResult {
                    status_msg: "Get the submit result error, please check your code, it may fail \
                                 to execute, or check your network"
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
        let pb = get_question_index(&idslug).await?;

        let json: Json = init_subit_list_grql(&pb.question_title_slug);

        let pat: submit_list::SubmissionData = fetch(
            &self.client,
            &USER_CONFIG.urls.graphql,
            Some(json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await?;

        Ok(pat.data.submission_list)
    }

    #[instrument(skip(self))]
    pub async fn test_code(&self, idslug: IdSlug) -> Result<(TestInfo, RunResult)> {
        let (code, pb) = join!(
            self.get_user_code(idslug.clone()),
            get_question_index(&idslug)
        );
        let ((code, test_case), pb) = (code?, pb?);
        debug!("code:\n{}", code);

        let mut json: Json = HashMap::new();
        json.insert("lang", USER_CONFIG.config.lang.clone());
        json.insert("question_id", pb.question_id.to_string());
        json.insert("typed_code", code);
        json.insert("data_input", test_case);

        let test_info: TestInfo = fetch(
            &self.client,
            &USER_CONFIG.mod_test(&pb.question_title_slug),
            Some(json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await?;

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

            let resp_json: RunResult = fetch(
                &self.client.clone(),
                &USER_CONFIG.mod_submissions(&test_info.interpret_id),
                None,
                SendMode::Get,
                self.headers.clone(),
            )
            .await?;
            if resp_json.state == "SUCCESS" {
                return Ok(resp_json);
            }

            if count > 9 {
                return Ok(RunResult {
                    status_msg: "Get the test result error, please check your network,or check \
                                 test case it may not correct"
                        .to_owned(),
                    ..Default::default()
                });
            }
            count += 1;
        }
    }

    /// Get user code as string(`code`, `test case`)
    pub async fn get_user_code(&self, idslug: IdSlug) -> Result<(String, String)> {
        let pb = get_question_index(&idslug).await?;
        let chf = CacheFile::build(&pb).await?;
        let (code, mut test_case) = chf.get_user_code(&idslug).await?;

        if test_case.is_empty() {
            test_case = self
                .get_qs_detail(idslug, false)
                .await?
                .example_testcases;
        }
        let (start, end, ..) = USER_CONFIG.get_lang_info();
        let code_re = Regex::new(&format!(r"(?s){}\n(?P<code>.*){}", start, end)).unwrap();

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
    use serde::de::DeserializeOwned;
    use tracing::trace;

    use crate::{leetcode::headers::Headers, Json};

    pub(super) enum SendMode {
        Get,
        Post,
    }

    pub(super) async fn fetch<T>(
        client: &Client,
        url: &str,
        json: Option<Json>,
        mode: SendMode,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
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

        resp.json::<T>()
            .await
            .map_err(|e| miette!("Error: {e}."))
    }
}
