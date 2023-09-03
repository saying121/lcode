mod graphqls;
pub mod qs_detail;
pub mod qs_index;
pub mod resps;

use std::{collections::HashMap, fmt::Display, sync::mpsc::Sender, time::Duration};

use self::{
    graphqls::*,
    leetcode_send::*,
    qs_detail::*,
    qs_index::QsIndex,
    resps::{run_res::RunResult, submit_list::SubmissionList, *},
};
use crate::{
    config::{
        global::{glob_user_config, CATEGORIES},
        Config, User,
    },
    dao::{get_question_index_exact, save_info::CacheFile},
    entities::{prelude::*, *},
    mytui::myevent::UserEvent,
};
use colored::Colorize;
use miette::{miette, Error, IntoDiagnostic, Result};
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use tokio::{fs::File, io::AsyncReadExt, join, task::spawn_blocking, time::sleep};
use tracing::{debug, error, info, instrument, trace};

#[derive(Debug, Clone)]
pub enum IdSlug {
    Id(u32),
    Slug(String),
}

impl Display for IdSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdSlug::Id(num) => num.fmt(f),
            IdSlug::Slug(slug) => slug.fmt(f),
        }
    }
}

pub type Json = HashMap<&'static str, String>;

/// interact with leetcode.com/cn
#[derive(Debug, Default)]
pub struct LeetCode {
    pub client: Client,
    pub headers: HeaderMap,
    pub user: User,
    db: DatabaseConnection,
}

impl LeetCode {
    /// Create a LeetCode instance and initialize some variables
    pub async fn new() -> Result<Self, Error> {
        let client = ClientBuilder::new()
            .gzip(true)
            .connect_timeout(Duration::from_secs(30))
            .build()
            .into_diagnostic()?;

        let user_handle = spawn_blocking(|| glob_user_config().to_owned());
        let (config, user_res, db) =
            join!(Config::new(), user_handle, crate::dao::conn_db());

        Ok(LeetCode {
            client,
            headers: config?.headers,
            user: user_res.into_diagnostic()?,
            db: db?,
        })
    }

    /// get leetcode index
    ///
    /// # Panics
    ///
    /// - json parser error
    ///
    /// # Errors
    ///
    /// - network error
    /// - leetcode url change
    /// - DbErr
    /// * `force`: when true will force update
    #[instrument(skip(self))]
    pub async fn sync_problem_index(&self) -> Result<(), Error> {
        for category in CATEGORIES {
            let all_pb_url = self.user.mod_all_pb_api(category);

            let resp_json = fetch(
                &self.client,
                &all_pb_url,
                None,
                SendMode::Get,
                self.headers.clone(),
            )
            .await?;

            // Get the part of the question
            let problems_json = resp_json
                .get("stat_status_pairs")
                .cloned()
                .unwrap_or_default()
                .as_array()
                .cloned()
                .unwrap();

            for problem in problems_json {
                debug!("deserialize :{}", problem);

                let pb: QsIndex =
                    serde_json::from_value(problem.clone()).into_diagnostic()?;

                let temp = Index::find_by_id(pb.stat.question_id)
                    .one(&self.db)
                    .await
                    .into_diagnostic()?;

                let pb_db = pb.to_active_model(category);
                if temp.is_some() {
                    Index::update(pb_db)
                        .exec(&self.db)
                        .await
                        .into_diagnostic()?;
                } else if !temp.is_some() {
                    Index::insert(pb_db)
                        .exec(&self.db)
                        .await
                        .into_diagnostic()?;
                }
            }
        }

        Ok(())
    }
    #[instrument(skip(self, tx))]
    pub async fn sync_index_with_state<'a>(
        &self,
        tx: Sender<UserEvent>,
    ) -> Result<(), Error> {
        for category in CATEGORIES {
            let all_pb_url = self.user.mod_all_pb_api(category);

            let resp_json = fetch(
                &self.client,
                &all_pb_url,
                None,
                SendMode::Get,
                self.headers.clone(),
            )
            .await?;

            // Get the part of the question
            let problems_json = resp_json
                .get("stat_status_pairs")
                .cloned()
                .unwrap_or_default()
                .as_array()
                .cloned()
                .unwrap();

            let mut cur_sync = 0;
            // prevent division by 0
            let total = problems_json.len().max(1);

            for problem in problems_json {
                debug!("deserialize :{}", problem);

                let pb: QsIndex =
                    serde_json::from_value(problem.clone()).into_diagnostic()?;

                let temp = Index::find_by_id(pb.stat.question_id)
                    .one(&self.db)
                    .await
                    .into_diagnostic()?;

                let pb_db = pb.to_active_model(category);
                if temp.is_some() {
                    Index::update(pb_db)
                        .exec(&self.db)
                        .await
                        .into_diagnostic()?;
                } else if !temp.is_some() {
                    Index::insert(pb_db)
                        .exec(&self.db)
                        .await
                        .into_diagnostic()?;
                }

                cur_sync += 1;

                // Update every 60 questions synced
                if cur_sync % 60 == 0 || cur_sync == total {
                    tx.send(UserEvent::Syncing((cur_sync, total, category.to_owned())))
                        .into_diagnostic()?;
                }
            }
        }

        tx.send(UserEvent::SyncDone)
            .into_diagnostic()?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn new_sync_index(&self) -> Result<()> {
        let url = &self.user.urls.graphql;
        let mut json: Json = HashMap::new();
        json.insert("query", init_pbsetlist_grql().join("\n"));

        json.insert(
            "variables",
            r#"{"skip":$skip,"limit":10,"filters":{}}"#.replace("$skip", "0"),
        );
        json.insert("operationName", "problemsetQuestionList".to_string());

        let resp_json = fetch(
            &self.client,
            url,
            Some(json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await?;
        debug!("full json: {:#?}", resp_json);
        Ok(())
    }

    /// Get the details of the problem, and if it's in the cache, use it.
    /// Write data to file.
    ///
    /// * `id`: id of the problem
    /// * `force`: when true, the cache will be re-fetched
    #[instrument(skip(self))]
    pub async fn get_qs_detail(
        &self,
        idslug: IdSlug,
        force: bool,
    ) -> Result<Question, Error> {
        let pb = get_question_index_exact(&idslug).await?;

        debug!("pb: {:?}", pb);

        let temp = Detail::find_by_id(pb.question_id)
            .one(&self.db)
            .await
            .into_diagnostic()?;

        #[allow(unused_assignments)]
        let mut detail = Question::default();

        if temp.is_some() && !force {
            let the_detail = temp.unwrap();
            detail = serde_json::from_str(&the_detail.content).unwrap_or_default();
        } else {
            let mut json: Json = HashMap::new();
            json.insert("query", init_qs_dt_grql().join("\n"));

            json.insert(
                "variables",
                r#"{"titleSlug": "$titleSlug"}"#
                    .replace("$titleSlug", &pb.question_title_slug),
            );
            json.insert("operationName", "getQuestion".to_string());
            trace!("get detail insert json: {:#?}", json);

            let pb_json = fetch(
                &self.client,
                &self.user.urls.graphql.to_string(),
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

            detail = Question::parser_question(pb_data, pb.question_title_slug);
            // detail = serde_json::from_value(pb_data).into_diagnostic()?;
            // detail.qs_slug = Some(pb.question_title_slug);

            let question_string = serde_json::to_string(&detail).unwrap_or_default();

            let pb_dt_model = detail::ActiveModel {
                id: ActiveValue::Set(pb.question_id),
                content: ActiveValue::Set(question_string),
            };

            if force && temp.is_some() {
                Detail::update(pb_dt_model)
                    .exec(&self.db)
                    .await
                    .into_diagnostic()?;
            } else {
                Detail::insert(pb_dt_model)
                    .exec(&self.db)
                    .await
                    .into_diagnostic()?;
            }
        }
        let chf = CacheFile::new(&idslug).await?;
        chf.write_to_file(detail.clone(), &self.user)
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
        json.insert("lang", self.user.lang.clone());
        json.insert("question_id", pb.question_id.to_string());
        json.insert("typed_code", code);

        trace!("submit insert json: {:#?}", json);

        let resp_json = fetch(
            &self.client,
            &self
                .user
                .mod_submit(&pb.question_title_slug),
            Some(json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await?;

        trace!("submit resp_json: {:?}", resp_json);

        let sub_id: SubmitInfo = serde_json::from_value(resp_json).map_err(|e| {
            miette!(
                "{}: {}, check your cookies or network.",
                "Error".color("red"),
                e
            )
        })?;
        trace!("out submit id: {}", sub_id.submission_id);

        let last_sub_result = self
            .get_one_submit_res(&sub_id)
            .await?;
        trace!("last submit result: {:#?}", last_sub_result);

        Ok((sub_id, last_sub_result))
    }

    /// Get one submit info
    ///
    /// # Example
    /// ```rust
    /// let a = leetcode::LeetCode::new().await?;
    /// let res = a.submit_code(IdSlug::Id(1)).await?;
    /// a.last_submit_res(res).await?;
    /// ```
    ///
    /// * `sub_id`: be fetch submission_id
    #[instrument(skip(self))]
    pub async fn get_one_submit_res(&self, sub_id: &SubmitInfo) -> Result<RunResult> {
        let test_res_url = self
            .user
            .mod_submissions(&sub_id.submission_id.to_string());
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
                    info!("waiting resp")
                }
            }

            if count > 9 {
                return Err(miette!(
                    "Get the submit result error, please check your code, \
                                   it may fail to execute, or check your network"
                ));
            }
            count += 1;
        }
    }

    /// Get all submission results for a question
    #[instrument(skip(self))]
    pub async fn all_submit_res(&self, idslug: IdSlug) -> Result<SubmissionList> {
        let pb = get_question_index_exact(&idslug).await?;

        let mut json: Json = HashMap::new();
        json.insert("query", init_subit_list_grql().join("\n"));
        json.insert(
            "variables",
            r#"{"questionSlug":"$Slug", "offset":0,"limit":$num,"lastKey":null,"status":null}"#
                .replace("$Slug", &pb.question_title_slug)
                .replace("$num", &self.user.num_sublist.to_string()),
        );
        json.insert("operationName", "submissionList".to_owned());

        let resp_json = fetch(
            &self.client,
            &self.user.urls.graphql,
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
        trace!("be serde submission list: {:#?}", be_serde);

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

        let mut json: Json = HashMap::new();
        json.insert("lang", self.user.lang.clone());
        json.insert("question_id", pb.question_id.to_string());
        json.insert("typed_code", code);
        json.insert("data_input", test_case);

        let resp_json = fetch(
            &self.client,
            &self
                .user
                .mod_test(&pb.question_title_slug),
            Some(json),
            SendMode::Post,
            self.headers.clone(),
        )
        .await?;

        trace!("test resp json: {:#?}", resp_json);

        let test_info: TestInfo = serde_json::from_value(resp_json).into_diagnostic()?;
        debug!("test info: {:#?}", test_info);

        let test_result = self
            .get_test_res(&test_info)
            .await?;
        trace!("test result: {:#?}", test_result);

        Ok((test_info, test_result))
    }

    /// Get the last submission results for a question
    async fn get_test_res(&self, test_info: &TestInfo) -> Result<RunResult> {
        let mut count = 0;
        loop {
            sleep(Duration::from_millis(700)).await;

            let resp_json = fetch(
                &self.client.to_owned(),
                &self
                    .user
                    .mod_submissions(&test_info.interpret_id),
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
                return Err(miette!(
                    "Get the test result error, please check your code,\
                    it may fail to execute, or check your network, \
                    or check test case it may not correct"
                ));
            }
            count += 1;
        }
    }

    /// Get user code as string
    async fn get_user_code(&self, idslug: IdSlug) -> Result<(String, String)> {
        let chf = CacheFile::new(&idslug).await?;

        let (code_file, test_case_file) =
            join!(File::open(chf.code_path), File::open(chf.test_case_path));
        let (mut code_file, mut test_case_file) = (
            code_file.map_err(|err| {
                miette!(
                    "Error: {}. There is no code file, \
                    maybe you changed the name, please get **{}** question detail again",
                    err,
                    idslug
                )
            })?,
            test_case_file.map_err(|err| {
                miette!(
                    "Error: {}. There is no test case file, \
                    maybe you changed the name, \
                    please remove relate file and get **{}** question detail again, \
                    or manual create a same name blank file",
                    err,
                    idslug
                )
            })?,
        );

        let mut code = "".to_string();
        let mut test_case = "".to_string();

        let (code_res, test_case_res) = join!(
            code_file.read_to_string(&mut code),
            test_case_file.read_to_string(&mut test_case)
        );
        let _ = (
            code_res.into_diagnostic()?,
            test_case_res.into_diagnostic()?,
        );

        // sometimes the test case file will be empty,
        // when get **2** question it's test case file is empty, bitch.
        if test_case.len() == 0 {
            test_case = self
                .get_qs_detail(idslug, false)
                .await?
                .example_testcases;
        }

        Ok((code, test_case))
    }
}

mod leetcode_send {
    use super::Json;
    use crate::config::Config;
    use miette::{miette, Error, IntoDiagnostic, Result};
    use reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    };
    use serde_json::Value;
    use tracing::trace;

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
    ) -> Result<Value, Error> {
        let headers = Config::mod_headers(headers, vec![("Referer", &url)])?;

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

        resp.json()
            .await
            .map_err(|e| miette!("Error: {}, check your cookies or network.", e))
    }
}
