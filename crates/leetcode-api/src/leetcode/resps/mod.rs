pub mod checkin;
pub mod pass_qs;
pub mod run_res;
pub mod submit_list;
pub mod user_data;

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
