use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

/// base info of question
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct QsIndex {
    #[serde(default)]
    pub stat: Stat,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub difficulty: Difficulty,
    #[serde(default)]
    pub paid_only: bool,
    #[serde(default)]
    pub is_favor: bool,
    #[serde(default)]
    pub frequency: u32,
    #[serde(default)]
    pub progress: u32,
}

impl QsIndex {
    pub fn to_active_model(self, category: &str) -> index::ActiveModel {
        index::ActiveModel {
            question_id: ActiveValue::Set(self.stat.question_id),
            question_article_live: ActiveValue::Set(self.stat.question_article_live),
            question_article_slug: ActiveValue::Set(self.stat.question_article_slug),
            question_article_has_video_solution: ActiveValue::Set(
                self.stat
                    .question_article_has_video_solution,
            ),
            question_title: ActiveValue::Set(self.stat.question_title),
            question_title_slug: ActiveValue::Set(self.stat.question_title_slug),
            question_hide: ActiveValue::Set(self.stat.question_hide),
            total_acs: ActiveValue::Set(self.stat.total_acs),
            total_submitted: ActiveValue::Set(self.stat.total_submitted),
            frontend_question_id: ActiveValue::Set(self.stat.frontend_question_id),
            is_new_question: ActiveValue::Set(self.stat.is_new_question),
            status: ActiveValue::Set(self.status),
            difficulty: ActiveValue::Set(self.difficulty.level),
            paid_only: ActiveValue::Set(self.paid_only),
            is_favor: ActiveValue::Set(self.is_favor),
            frequency: ActiveValue::Set(self.frequency),
            progress: ActiveValue::Set(self.progress),
            category: ActiveValue::Set(category.to_owned()),
            pass_rate: ActiveValue::Set(Some(
                self.stat.total_acs as f64 / self.stat.total_submitted as f64 * 100.0,
            )),
        }
    }
}

use question::*;

use crate::entities::index;

pub mod question {
    use serde::{Deserialize, Deserializer, Serialize};

    use crate::config::global::glob_user_config;

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct Stat {
        pub question_id: u32,
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
        pub total_acs: u32,
        pub total_submitted: u32,
        #[serde(default, deserialize_with = "my_id_deserialize")]
        pub frontend_question_id: String,
        pub is_new_question: bool,
    }

    fn my_id_deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        if glob_user_config().url_suffix == "cn" {
            let s = String::deserialize(deserializer)?;

            return Ok(s);
        } else {
            let s = u32::deserialize(deserializer)?;

            return Ok(s.to_string());
        }
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct Difficulty {
        pub level: u32,
    }
}
