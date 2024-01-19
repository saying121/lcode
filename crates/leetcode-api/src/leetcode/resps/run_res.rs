use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TestInfo {
    #[serde(default)]
    pub interpret_id:          String,
    #[serde(default)]
    pub test_case:             String,
    #[serde(default)]
    pub interpret_expected_id: String,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubmitInfo {
    #[serde(default)]
    pub submission_id: u32,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct RunResult {
    #[serde(default)]
    pub elapsed_time: u32,
    #[serde(default)]
    pub finished:     bool,
    // pub expected_elapsed_time: u32,
    // pub expected_lang: String,
    // pub expected_memory: u128,
    // pub expected_run_success: bool,
    // pub expected_status_code: i32,
    // pub expected_status_runtime: String,
    // pub expected_std_output_list: Vec<String>,
    // pub expected_task_finish_time: u128,
    // pub expected_task_name: String,
    // pub fast_submit: bool,
    #[serde(default)]
    pub task_name:    String,

    #[serde(default)]
    pub status_code: i64,
    #[serde(default)]
    pub status_msg:  String,

    #[serde(default)]
    pub question_id:     String,
    #[serde(default)]
    pub std_output:      String,
    #[serde(default)]
    pub expected_output: String,
    #[serde(default)]
    pub last_testcase:   String,

    #[serde(default)]
    pub code_answer:          Vec<String>,
    // #[serde(default)]
    // pub code_output: String, // test:vec,submit:string, delete the field
    #[serde(default)]
    pub compare_result:       String,
    #[serde(default)]
    pub correct_answer:       bool,
    #[serde(default)]
    pub expected_code_answer: Vec<String>,
    #[serde(default)]
    pub expected_code_output: Vec<String>,

    #[serde(default)]
    pub pretty_lang: String,
    #[serde(default)]
    pub lang:        String,

    #[serde(default)]
    pub memory:            u64,
    #[serde(default)]
    pub status_memory:     String,
    #[serde(default)]
    pub memory_percentile: Option<f64>,

    #[serde(default)]
    pub status_runtime:     String,
    #[serde(default)]
    pub runtime_percentile: Option<f64>,
    #[serde(default)]
    pub run_success:        bool,

    #[serde(default)]
    pub state: String,

    #[serde(default)]
    pub std_output_list: Vec<String>,
    #[serde(default)]
    pub submission_id:   String,

    #[serde(default)]
    pub task_finish_time: u64,

    #[serde(default)]
    pub total_correct:   Option<u64>,
    #[serde(default)]
    pub total_testcases: Option<u64>,

    // runtime error
    #[serde(default)]
    pub full_runtime_error: String,
    #[serde(default)]
    pub runtime_error:      String,

    // compile error
    #[serde(default)]
    pub compile_error:      String,
    #[serde(default)]
    pub full_compile_error: String,
}
