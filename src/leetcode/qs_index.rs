use sea_orm::{sea_query, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use tracing::error;

use question::*;

use crate::{
    dao::glob_db,
    entities::{index, prelude::*},
};

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

pub mod question {
    use serde::{Deserialize, Deserializer, Serialize};

    use crate::config::global::glob_user_config;

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct Stat {
        pub question_id: u32,
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
    }

    fn my_id_deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        if glob_user_config().url_suffix == "cn" {
            let s = String::deserialize(deserializer)?;

            Ok(s)
        } else {
            let s = u32::deserialize(deserializer)?;

            Ok(s.to_string())
        }
    }

    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub struct Difficulty {
        pub level: u32,
    }
}

impl QsIndex {
    pub fn into_active_model(self, category: &str) -> index::ActiveModel {
        index::ActiveModel {
            question_id: Set(self.stat.question_id),
            question_title: Set(self.stat.question_title),
            question_title_slug: Set(self.stat.question_title_slug),
            total_acs: Set(self.stat.total_acs),
            total_submitted: Set(self.stat.total_submitted),
            frontend_question_id: Set(self.stat.frontend_question_id),
            status: Set(self.status),
            difficulty: Set(self.difficulty.level),
            paid_only: Set(self.paid_only),
            is_favor: Set(self.is_favor),
            frequency: Set(self.frequency),
            progress: Set(self.progress),
            category: Set(category.to_owned()),
            pass_rate: Set(Some(
                f64::from(self.stat.total_acs) / f64::from(self.stat.total_submitted)
                    * 100.0,
            )),
        }
    }
    pub async fn insert_to_db(self, category: &str) {
        match Index::insert(self.into_active_model(category))
            .on_conflict(
                sea_query::OnConflict::column(index::Column::QuestionId)
                    .update_columns([
                        index::Column::QuestionTitle,
                        index::Column::QuestionTitleSlug,
                        index::Column::QuestionId,
                        index::Column::FrontendQuestionId,
                        index::Column::TotalAcs,
                        index::Column::TotalSubmitted,
                        index::Column::Status,
                        index::Column::Difficulty,
                        index::Column::PaidOnly,
                        index::Column::IsFavor,
                        index::Column::Frequency,
                        index::Column::Progress,
                        index::Column::Category,
                        index::Column::PassRate,
                    ])
                    .to_owned(),
            )
            .exec(glob_db())
            .await
        {
            Ok(_) => {}
            Err(err) => error!("{}", err),
        };
    }
}
