use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub struct SubmissionData {
    #[serde(default)]
    data: SubmissionDataInner,
}

impl SubmissionData {
    pub fn submission_list(self) -> SubmissionList {
        self.data.submission_list
    }
}
#[derive(Deserialize, Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
struct SubmissionDataInner {
    #[serde(default, alias = "submissionList")]
    submission_list: SubmissionList,
}

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub struct SubmissionList {
    #[serde(default, alias = "lastKey")]
    pub(crate) last_key: Option<String>,
    #[serde(default, alias = "hasNext")]
    pub(crate) has_next: bool,
    #[serde(default)]
    pub(crate) submissions: Vec<Submission>,
}

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub struct Submission {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default, alias = "statusDisplay")]
    pub status_display: Option<String>,
    #[serde(default)]
    pub lang: String,
    #[serde(default, alias = "langName")]
    pub lang_name: String,
    #[serde(default)]
    pub runtime: String,
    #[serde(default)]
    pub timestamp: String,
    #[serde(default)]
    pub url: String,
    #[serde(default, alias = "isPending")]
    pub is_pending: String,
    #[serde(default)]
    pub memory: String,
    #[serde(default, alias = "submissionComment")]
    pub submission_comment: Option<String>,
}
