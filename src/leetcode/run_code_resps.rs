use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum Submissions {
    Success(SubmissionDetail),
    Fail(SubmissionFail),
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct SubmissionFail {
    compile_error: String,
    fast_submit: bool,
    finished: bool,
    full_compile_error: String,
    lang: String,
    memory: u128,
    memory_percentile: Option<f64>,
    pretty_lang: String,
    question_id: String,
    run_success: bool,
    runtime_percentile: Option<f64>,
    state: String,
    status_code: i32,
    status_memory: String,
    status_msg: String,
    status_runtime: String,
    submission_id: String,
    task_finish_time: u128,
    task_name: String,
    total_correct: Option<u32>,
    total_testcases: Option<u32>,
}
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

pub mod list_nest {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Submission {
        id: String,
        title: String,
        status: Option<String>,
        #[serde(alias = "statusDisplay")]
        status_display: Option<String>,
        lang: String,
        #[serde(alias = "langName")]
        lang_name: String,
        runtime: String,
        timestamp: String,
        url: String,
        #[serde(alias = "isPending")]
        is_pending: String,
        memory: String,
        #[serde(alias = "submissionComment")]
        submission_comment: Option<String>,
    }
}
////////////////////////////////////////////////////
// test code

#[derive(Deserialize, Serialize, Debug)]
pub struct TestInfo {
    pub interpret_id: String,
    pub test_case: String,
    pub interpret_expected_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
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
