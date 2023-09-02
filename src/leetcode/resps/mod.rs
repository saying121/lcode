use serde::{Deserialize, Serialize};

pub mod run_res;
pub mod submit_list;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct TestInfo {
    pub interpret_id: String,
    pub test_case: String,
    pub interpret_expected_id: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct SubmitInfo {
    pub submission_id: u32,
}
