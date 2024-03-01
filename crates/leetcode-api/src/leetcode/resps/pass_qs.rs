use lcode_config::config::{global::G_USER_CONFIG, user_nest::Suffix};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct Passdata {
    #[serde(default, deserialize_with = "deserialize_data")]
    pub data: PassData,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct DataCom {
    #[serde(default, alias = "allQuestionsCount")]
    all_questions_count: Vec<DifficultyPass>,
    #[serde(default, alias = "matchedUser")]
    matched_user:        MatchedUser,
}

impl DataCom {
    fn infos(&self) -> Vec<String> {
        let mut res = Vec::with_capacity(4);
        if let Some(v) = &self.matched_user.submit_stats {
            for i in &v.ac_submission_num {
                res.push(format!("{}: {}", i.difficulty, i.count));
            }
        }
        res
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct MatchedUser {
    #[serde(default, alias = "submitStats")]
    submit_stats: Option<SubmitStats>,
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct SubmitStats {
    #[serde(default, alias = "acSubmissionNum")]
    ac_submission_num:    Vec<DifficultyPass>,
    #[serde(default, alias = "totalSubmissionNum")]
    total_submission_num: Vec<DifficultyPass>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum PassData {
    Cn(DataCn),
    Com(DataCom),
}

impl PassData {
    pub fn infos(&self) -> Vec<String> {
        match self {
            Self::Cn(v) => v
                .user_profile_user_question_progress
                .infos(),
            Self::Com(v) => v.infos(),
        }
    }
}

impl Default for PassData {
    fn default() -> Self {
        Self::Cn(DataCn::default())
    }
}

pub fn deserialize_data<'de, D>(deserializer: D) -> Result<PassData, D::Error>
where
    D: Deserializer<'de>,
{
    let res = match G_USER_CONFIG.config.url_suffix {
        Suffix::Cn => {
            let pat = DataCn::deserialize(deserializer)?;
            PassData::Cn(pat)
        },
        Suffix::Com => {
            let pat = DataCom::deserialize(deserializer)?;
            PassData::Com(pat)
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
    user_profile_user_question_progress:     UserProfileUserQuestionProgress,
    #[serde(default, alias = "userProfileUserQuestionSubmitStats")]
    user_profile_user_question_submit_stats: UserProfileUserQuestionSubmitStats,
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
    num_accepted_questions:  Vec<DifficultyPass>,
    #[serde(default, alias = "numFailedQuestions")]
    num_failed_questions:    Vec<DifficultyPass>,
    #[serde(default, alias = "numUntouchedQuestions")]
    num_untouched_questions: Vec<DifficultyPass>,
}

impl UserProfileUserQuestionProgress {
    fn infos(&self) -> Vec<String> {
        let mut res = Vec::with_capacity(4);
        let mut all = 0;
        for i in &self.num_accepted_questions {
            res.push(format!("{}: {}", i.difficulty, i.count));
            all += i.count;
        }
        res.push(format!("All: {}", all));
        res
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct DifficultyPass {
    difficulty: String,
    count:      u32,
}
