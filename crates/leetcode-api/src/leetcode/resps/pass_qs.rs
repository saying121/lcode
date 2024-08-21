use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct Passdata {
    #[serde(default)]
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
    matched_user: MatchedUser,
}

impl DataCom {
    fn info(&self) -> Vec<String> {
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
    ac_submission_num: Vec<DifficultyPass>,
    #[serde(default, alias = "totalSubmissionNum")]
    total_submission_num: Vec<DifficultyPass>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PassData {
    Cn(DataCn),
    Com(DataCom),
}

impl PassData {
    pub fn info(&self) -> Vec<String> {
        match self {
            Self::Cn(v) => v
                .user_profile_user_question_progress
                .info(),
            Self::Com(v) => v.info(),
        }
    }
}

impl Default for PassData {
    fn default() -> Self {
        Self::Cn(DataCn::default())
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct DataCn {
    #[serde(default, alias = "userProfileUserQuestionProgress")]
    user_profile_user_question_progress: UserProfileUserQuestionProgress,
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
    num_accepted_questions: Vec<DifficultyPass>,
    #[serde(default, alias = "numFailedQuestions")]
    num_failed_questions: Vec<DifficultyPass>,
    #[serde(default, alias = "numUntouchedQuestions")]
    num_untouched_questions: Vec<DifficultyPass>,
}

impl UserProfileUserQuestionProgress {
    fn info(&self) -> Vec<String> {
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
    count: u32,
}
