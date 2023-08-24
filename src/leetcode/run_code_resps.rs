use std::fmt::Display;

use serde::{Deserialize, Serialize};

// submit
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct SubmissionDetail {
    status_code: i32,
    lang: String,
    run_success: bool,
    status_runtime: String,
    memory: u128,
    question_id: String,
    elapsed_time: u32,
    compare_result: String,
    code_output: String,
    std_output: String,
    last_testcase: String,
    expected_output: String,
    task_finish_time: u128,
    task_name: String,
    finished: bool,
    status_msg: String,
    state: String,
    fast_submit: bool,
    total_correct: Option<u32>,
    total_testcases: Option<u32>,
    submission_id: String,
    runtime_percentile: Option<f64>,
    status_memory: String,
    memory_percentile: Option<f64>,
    pretty_lang: String,
    // input_formatted: String,
    // input: String,
}

impl Render for SubmissionDetail {
    fn to_tui_vec(&self) -> Vec<String> {
        vec![
            format!("# Submission Detail"),
            format!("• Status: {msg}", msg = self.status_msg),
            format!(
                "• Total Correct: {crt}",
                crt = self
                    .total_correct
                    .unwrap_or_default()
            ),
            format!(
                "• Total test case: {t_cases}",
                t_cases = self
                    .total_testcases
                    .unwrap_or_default()
            ),
            format!("• Memory: {mem}", mem = self.status_memory),
            format!(
                "• Memory exceeds: {p_mem}%",
                p_mem = self
                    .memory_percentile
                    .unwrap_or_default()
            ),
            format!("• Runtime: {rtm}", rtm = self.status_runtime),
            format!(
                "• Fast than: {rtm_p}%",
                rtm_p = self
                    .runtime_percentile
                    .unwrap_or_default()
            ),
            format!("• StdOut: {out}", out = self.std_output),
            format!("• Expect Out: {e_ot}", e_ot = self.expected_output),
            format!("• Last Test Case(Fail): {ltc}", ltc = self.last_testcase),
        ]
    }
    fn to_tui_mdvec(&self, _width: usize) -> Vec<String> {
        vec![]
    }
    fn to_rendered_str(&self, _col: u16, _row: u16) -> miette::Result<String> {
        Ok("".to_string())
    }
}

impl Display for SubmissionDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(
            "# Submission Detail \n\
                * Status: {msg} \n\
                * Total Correct: {crt} \n\
                * Total test case: {t_cases} \n\
                * Memory: {mem} \n\
                * Memory exceeds: {p_mem}% \n\
                * Runtime: {rtm} \n\
                * Fast than: {rtm_p}% \n\
                * StdOut: {out} \n\
                * Expect Out: {e_ot} \n\
                * Last Test Case(Fail): \n\
                {ltc}",
            msg = self.status_msg,
            crt = self
                .total_correct
                .unwrap_or_default(),
            t_cases = self
                .total_testcases
                .unwrap_or_default(),
            mem = self.status_memory,
            p_mem = self
                .memory_percentile
                .unwrap_or_default(),
            rtm = self.status_runtime,
            rtm_p = self
                .runtime_percentile
                .unwrap_or_default(),
            out = self.std_output,
            e_ot = self.expected_output,
            ltc = self.last_testcase
        )
        .fmt(f)
    }
}

////////////////////////////////////////////////////
// submit list

#[derive(Deserialize, Serialize, Debug)]
pub struct SubmissionList {
    #[serde(alias = "lastKey")]
    last_key: String,
    #[serde(alias = "hasNext")]
    has_next: bool,
    submissions: Vec<list_nest::Submission>,
}
use tabled::{
    builder::Builder,
    settings::{style::Style, themes::ColumnNames},
};

use crate::{config::global::global_user_config, render::Render};

impl Display for SubmissionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let user = global_user_config();

        let mut subs = vec![];
        let mut temp = vec![];

        for i in 0..user.column {
            temp.push(i.to_string());
        }

        subs.push(temp.clone());
        temp.clear();

        for submission in &self.submissions {
            temp.push(submission.to_string());
            if temp.len() >= user.column {
                subs.push(temp.clone());
                temp.clear();
            }
        }
        if temp.len() > 0 {
            subs.push(temp.clone());
        }

        let mut table = Builder::from(subs).build();
        table
            .with(Style::modern())
            .with(ColumnNames::default());

        format!("{}", table).fmt(f)
    }
}

pub mod list_nest {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Submission {
        pub id: String,
        pub title: String,
        pub status: Option<String>,
        #[serde(alias = "statusDisplay")]
        pub status_display: Option<String>,
        pub lang: String,
        #[serde(alias = "langName")]
        pub lang_name: String,
        pub runtime: String,
        pub timestamp: String,
        pub url: String,
        #[serde(alias = "isPending")]
        pub is_pending: String,
        pub memory: String,
        #[serde(alias = "submissionComment")]
        pub submission_comment: Option<String>,
    }

    impl Display for Submission {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            format!(
                "• Title: {title} \n\
                • ID: {id} \n\
                • Lang: {lang} \n\
                • Status: {status} \n\
                • Runtime: {tim} \n\
                • Memory: {mem} \n\
                • Comment: {cmt} \n\
                ",
                cmt = self
                    .submission_comment
                    .as_ref()
                    .map(|v| v.as_str())
                    .unwrap_or_default(),
                id = self.id,
                title = self.title,
                lang = self.lang_name,
                status = self
                    .status_display
                    .as_ref()
                    .map(|v| v.as_str())
                    .unwrap_or_default(),
                tim = self.runtime,
                mem = self.memory,
            )
            .fmt(f)
        }
    }
}
////////////////////////////////////////////////////
// test code

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct TestInfo {
    pub interpret_id: String,
    pub test_case: String,
    pub interpret_expected_id: String,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct TestResult {
    code_answer: Vec<String>,
    code_output: Vec<String>,
    compare_result: String,
    correct_answer: bool,
    elapsed_time: u32,
    expected_code_answer: Vec<String>,
    expected_code_output: Vec<String>,
    expected_elapsed_time: u32,
    expected_lang: String,
    expected_memory: u128,
    expected_run_success: bool,
    expected_status_code: i32,
    expected_status_runtime: String,
    expected_std_output_list: Vec<String>,
    expected_task_finish_time: u128,
    expected_task_name: String,
    fast_submit: bool,
    lang: String,
    memory: u128,
    memory_percentile: Option<f64>,
    pretty_lang: String,
    run_success: bool,
    runtime_percentile: Option<f64>,
    state: String,
    status_code: i32,
    status_memory: String,
    status_msg: String,
    status_runtime: String,
    std_output_list: Vec<String>,
    submission_id: String,
    task_finish_time: u128,
    task_name: String,
    total_correct: u32,
    total_testcases: u32,
}

impl Render for TestResult {
    fn to_tui_vec(&self) -> Vec<String> {
        vec![
            format!("# Test Result"),
            format!("• Pass: {correct} ", correct = self.correct_answer),
            format!("• Lang: {lang} ", lang = self.pretty_lang),
            format!("• Total correct {tc} ", tc = self.total_correct),
            format!("• Total Testcases {tt} ", tt = self.total_testcases),
            format!("• Memory: {mem} ", mem = self.status_memory),
            format!("• Runtime: {tim} ", tim = self.status_runtime),
            format!(
                "* Your Answer: {ans} ",
                ans = self
                    .code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("")
            ),
            format!(
                "* Correct Answer: {c_ans} ",
                c_ans = self
                    .expected_code_answer
                    .iter()
                    .map(|v| format!("    * {}", v))
                    .collect::<Vec<String>>()
                    .join("")
            ),
        ]
    }
    fn to_tui_mdvec(&self, _width: usize) -> Vec<String> {
        vec![]
    }
    fn to_rendered_str(&self, _col: u16, _row: u16) -> miette::Result<String> {
        Ok("".to_string())
    }
}

impl Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(
            "# Test Result \n\
                * Pass: {correct} \n\
                * Lang: {lang} \n\
                * Total correct {tc} \n\
                * Total Testcases {tt} \n\
                * Memory: {mem} \n\
                * Runtime: {tim} \n\
                * Your Answer: \n{ans} \n\
                * Correct Answer: \n{c_ans} ",
            lang = self.pretty_lang,
            tc = self.total_correct,
            tt = self.total_testcases,
            correct = self.correct_answer,
            mem = self.status_memory,
            tim = self.status_runtime,
            ans = self
                .code_answer
                .iter()
                .map(|v| format!("    * {}", v))
                .collect::<Vec<String>>()
                .join("\n"),
            c_ans = self
                .expected_code_answer
                .iter()
                .map(|v| format!("    * {}", v))
                .collect::<Vec<String>>()
                .join("\n")
        )
        .fmt(f)
    }
}
