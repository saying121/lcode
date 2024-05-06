use std::time::Duration;

use lcode_config::global::G_USER_CONFIG;
use miette::Result;
use regex::Regex;
use tokio::{join, time::sleep};
use tracing::{debug, trace};

use crate::{
    dao::{query::Query, save_info::FileInfo},
    leetcode::{
        graphqls::init_subit_list_grql,
        leetcode_send::fetch,
        resps::{
            run_res::*,
            submit_list::{SubmissionData, SubmissionList},
        },
        IdSlug, LeetCode,
    },
    Json,
};

impl LeetCode {
    /// submit code by id or slug, once submit one question
    ///
    /// * `idslug`: id or slug
    pub async fn submit_code(&self, idslug: IdSlug) -> Result<(SubmitInfo, RunResult)> {
        let (code, pb) = join!(
            self.get_user_code(idslug.clone()),
            Query::get_question_index(&idslug)
        );
        let ((code, _), pb) = (code?, pb?);

        let mut json: Json = Json::new();
        json.insert("lang", G_USER_CONFIG.config.lang.clone());
        json.insert("question_id", pb.question_id.to_string());
        json.insert("typed_code", code);

        trace!("submit insert json: {:#?}", json);

        let sub_info: SubmitInfo = match fetch(
            &self.client,
            &G_USER_CONFIG
                .urls
                .mod_submit(&pb.question_title_slug),
            Some(&json),
            self.headers.clone(),
        )
        .await
        {
            Ok(it) => it,
            Err(err) => {
                return Ok((
                    SubmitInfo::default(),
                    RunResultBuild::default()
                        .set_status_msg(err.to_string())
                        .build(),
                ));
            },
        };

        let last_sub_result = self
            .get_one_submit_res(&sub_info)
            .await?;
        debug!("last submit result: {:#?}", last_sub_result);

        Ok((sub_info, last_sub_result))
    }

    /// Get one submit info
    ///
    /// * `sub_id`: be fetch `submission_id`
    pub async fn get_one_submit_res(&self, sub_id: &SubmitInfo) -> Result<RunResult> {
        let test_res_url = G_USER_CONFIG
            .urls
            .mod_submissions(&sub_id.submission_id().to_string());
        trace!("start get last submit detail");

        for _ in 0..9 {
            sleep(Duration::from_millis(700)).await;

            let resp_json: RunResult =
                fetch(&self.client, &test_res_url, None, self.headers.clone()).await?;
            if resp_json.success() {
                return Ok(resp_json);
            }
        }
        Ok(RunResultBuild::default()
            .set_status_msg(
                "Get the submit result error, please check your code, it may fail to execute, or \
                 check your network"
                    .to_owned(),
            )
            .build())
    }

    /// Get all submission results for a question
    pub async fn all_submit_res(&self, idslug: IdSlug) -> Result<SubmissionList> {
        let pb = Query::get_question_index(&idslug).await?;

        let json: Json = init_subit_list_grql(&pb.question_title_slug);

        let pat: SubmissionData = fetch(
            &self.client,
            &G_USER_CONFIG.urls.graphql,
            Some(&json),
            self.headers.clone(),
        )
        .await?;

        Ok(pat.submission_list())
    }

    pub async fn test_code(&self, idslug: IdSlug) -> Result<(TestInfo, RunResult)> {
        let (code, pb) = join!(
            self.get_user_code(idslug.clone()),
            Query::get_question_index(&idslug)
        );
        let ((code, test_case), pb) = (code?, pb?);
        debug!("code:\n{}", code);

        let mut json: Json = Json::new();
        json.insert("lang", G_USER_CONFIG.config.lang.clone());
        json.insert("question_id", pb.question_id.to_string());
        json.insert("typed_code", code);
        json.insert("data_input", test_case);

        let test_info: TestInfo = match fetch(
            &self.client,
            &G_USER_CONFIG
                .urls
                .mod_test(&pb.question_title_slug),
            Some(&json),
            self.headers.clone(),
        )
        .await
        {
            Ok(it) => it,
            Err(err) => {
                return Ok((
                    TestInfo::default(),
                    RunResultBuild::default()
                        .set_status_msg(err.to_string())
                        .build(),
                ));
            },
        };

        let test_result = self.get_test_res(&test_info).await?;

        Ok((test_info, test_result))
    }

    /// Get the last submission results for a question
    async fn get_test_res(&self, test_info: &TestInfo) -> Result<RunResult> {
        for _ in 0..9 {
            sleep(Duration::from_millis(700)).await;

            let resp_json: RunResult = fetch(
                &self.client.clone(),
                &G_USER_CONFIG
                    .urls
                    .mod_submissions(test_info.interpret_id()),
                None,
                self.headers.clone(),
            )
            .await?;
            if resp_json.success() {
                return Ok(resp_json);
            }
        }
        Ok(RunResultBuild::default()
            .set_status_msg(
                "Get the test result error, please check your network,or check test case it may \
                 not correct"
                    .to_owned(),
            )
            .build())
    }

    /// Get user code as string(`code`, `test case`)
    pub async fn get_user_code(&self, idslug: IdSlug) -> Result<(String, String)> {
        let pb = Query::get_question_index(&idslug).await?;
        let chf = FileInfo::build(&pb).await?;
        let (code, mut test_case) = chf.get_user_code(&idslug).await?;

        if test_case.is_empty() {
            test_case = self
                .get_qs_detail(idslug, false)
                .await?
                .example_testcases;
        }
        let (start, end, ..) = G_USER_CONFIG.get_lang_info();
        let code_re = Regex::new(&format!(r"(?s){}\n(?P<code>.*){}", start, end))
            .expect("get_user_code regex new failed");

        // sep code just get needed
        #[allow(clippy::option_if_let_else)]
        let res = match code_re.captures(&code) {
            Some(val) => val["code"].to_owned(),
            None => code,
        };

        Ok((res, test_case))
    }
}
