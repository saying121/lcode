use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TestInfo {
    #[serde(default)]
    interpret_id: String,
    #[serde(default)]
    test_case: String,
    #[serde(default)]
    interpret_expected_id: String,
}

impl TestInfo {
    pub fn interpret_id(&self) -> &str {
        &self.interpret_id
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubmitInfo {
    #[serde(default)]
    submission_id: u32,
}

impl SubmitInfo {
    pub const fn submission_id(&self) -> u32 {
        self.submission_id
    }
}

#[derive(Deserialize, Serialize)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
pub struct RunResult {
    #[serde(default)]
    pub elapsed_time: u32,
    #[serde(default)]
    pub finished: bool,

    // #[serde(default)]
    // pub expected_elapsed_time: u32,
    // #[serde(default)]
    // pub expected_lang: String,
    // #[serde(default)]
    // pub expected_memory: u128,
    // #[serde(default)]
    // pub expected_run_success: bool,
    // #[serde(default)]
    // pub expected_status_code: i32,
    // #[serde(default)]
    // pub expected_status_runtime: String,
    // #[serde(default)]
    // pub expected_std_output_list: Vec<String>,
    // #[serde(default)]
    // pub expected_task_finish_time: u128,
    // #[serde(default)]
    // pub expected_task_name: String,
    // #[serde(default)]
    // pub fast_submit: bool,
    #[serde(default)]
    pub task_name: String,

    #[serde(default)]
    pub status_code: i64,
    #[serde(default)]
    pub status_msg: String,

    #[serde(default)]
    pub question_id: String,
    #[serde(default)]
    pub std_output: String,
    #[serde(default)]
    pub expected_output: String,
    #[serde(default)]
    pub last_testcase: String,

    #[serde(default)]
    pub code_answer: Vec<String>,
    // #[serde(default)]
    // pub code_output: String, // test:vec,submit:string, delete the field
    #[serde(default)]
    pub compare_result: String,
    #[serde(default)]
    pub correct_answer: bool,
    #[serde(default)]
    pub expected_code_answer: Vec<String>,
    #[serde(default)]
    pub expected_code_output: Vec<String>,

    #[serde(default)]
    pub pretty_lang: String,
    #[serde(default)]
    pub lang: String,

    #[serde(default)]
    pub memory: u64,
    #[serde(default)]
    pub status_memory: String,
    #[serde(default)]
    pub memory_percentile: Option<f64>,

    #[serde(default)]
    pub status_runtime: String,
    #[serde(default)]
    pub runtime_percentile: Option<f64>,
    #[serde(default)]
    pub run_success: bool,

    #[serde(default)]
    pub state: String,

    #[serde(default)]
    pub std_output_list: Vec<String>,
    #[serde(default)]
    pub submission_id: String,

    #[serde(default)]
    pub task_finish_time: u64,

    #[serde(default)]
    pub total_correct: Option<u64>,
    #[serde(default)]
    pub total_testcases: Option<u64>,

    // runtime error
    #[serde(default)]
    pub full_runtime_error: String,
    #[serde(default)]
    pub runtime_error: String,

    // compile error
    #[serde(default)]
    pub compile_error: String,
    #[serde(default)]
    pub full_compile_error: String,
}

impl RunResult {
    pub fn success(&self) -> bool {
        &self.state == "SUCCESS"
    }
    pub fn getting(&self) -> bool {
        &self.state == "STARTED"
    }

    pub fn total_correct(&self) -> u64 {
        self.total_correct.unwrap_or_default()
    }

    pub fn total_testcases(&self) -> u64 {
        self.total_testcases.unwrap_or_default()
    }

    pub fn memory_percentile(&self) -> f64 {
        self.memory_percentile
            .unwrap_or_default()
    }

    pub fn runtime_percentile(&self) -> f64 {
        self.runtime_percentile
            .unwrap_or_default()
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct RunResultBuild {
    inner: RunResult,
}

impl RunResultBuild {
    pub fn set_status_msg(mut self, status_msg: String) -> Self {
        self.inner.status_msg = status_msg;
        self
    }
    pub fn build(self) -> RunResult {
        self.inner
    }
}
