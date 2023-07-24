use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Problem {
    pub stat: Stat,
    pub status: Option<String>,
    pub difficulty: Difficulty,
    pub paid_only: bool,
    pub is_favor: bool,
    pub frequency: u64,
    pub progress: u64,
}

use question::*;

pub mod question {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Stat {
        pub question_id: u64,
        #[serde(rename = "question__article__live")]
        pub question_article_live: Option<bool>,
        #[serde(rename = "question__article__slug")]
        pub question_article_slug: Option<String>,
        #[serde(rename = "question__article__has_video_solution")]
        pub question_article_has_video_solution: Option<bool>,
        #[serde(rename = "question__title")]
        pub question_title: String,
        #[serde(rename = "question__title_slug")]
        pub question_title_slug: String,
        #[serde(rename = "question__hide")]
        pub question_hide: bool,
        pub total_acs: u64,
        pub total_submitted: u64,
        pub frontend_question_id: u64,
        pub is_new_question: bool,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Difficulty {
        pub level: u64,
    }
}
