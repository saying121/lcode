use lcode_config::config::{global::G_USER_CONFIG, user_nest::Suffix};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct PassData {
    #[serde(default, deserialize_with = "deserialize_data")]
    pub data: Data,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct DataCom {
    #[serde(default, alias = "allQuestionsCount")]
    pub all_questions_count: Vec<DifficultyPass>,
    #[serde(default, alias = "matchedUser")]
    pub matched_user:        MatchedUser,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct MatchedUser {
    #[serde(default, alias = "submitStats")]
    pub submit_stats: SubmitStats,
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct SubmitStats {
    #[serde(default, alias = "acSubmissionNum")]
    pub ac_submission_num:    Vec<DifficultyPass>,
    #[serde(default, alias = "totalSubmissionNum")]
    pub total_submission_num: Vec<DifficultyPass>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum Data {
    Cn(DataCn),
    Com(DataCom),
}

impl Default for Data {
    fn default() -> Self {
        Self::Cn(DataCn::default())
    }
}

pub fn deserialize_data<'de, D>(deserializer: D) -> Result<Data, D::Error>
where
    D: Deserializer<'de>,
{
    let res = match G_USER_CONFIG.config.url_suffix {
        Suffix::Cn => {
            let pat = DataCn::deserialize(deserializer)?;
            Data::Cn(pat)
        },
        Suffix::Com => {
            let pat = DataCom::deserialize(deserializer)?;
            Data::Com(pat)
        },
    };

    Ok(res)
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct DataCn {
    #[serde(default, alias = "userProfileUserQuestionProgress")]
    pub user_profile_user_question_progress:     UserProfileUserQuestionProgress,
    #[serde(default, alias = "userProfileUserQuestionSubmitStats")]
    pub user_profile_user_question_submit_stats: UserProfileUserQuestionSubmitStats,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct UserProfileUserQuestionSubmitStats {
    #[serde(default, alias = "acSubmissionNum")]
    ac_submission_num: Vec<DifficultyPass>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct UserProfileUserQuestionProgress {
    #[serde(default, alias = "numAcceptedQuestions")]
    pub num_accepted_questions:  Vec<DifficultyPass>,
    #[serde(default, alias = "numFailedQuestions")]
    pub num_failed_questions:    Vec<DifficultyPass>,
    #[serde(default, alias = "numUntouchedQuestions")]
    pub num_untouched_questions: Vec<DifficultyPass>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct DifficultyPass {
    pub difficulty: String,
    pub count:      u32,
}
