use sea_orm::{sea_query::OnConflict, EntityTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};
use tracing::error;

use question::*;

use crate::{
    dao::{glob_db, InsertToDB},
    entities::{index, prelude::*},
};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Problems {
    pub user_name: String,
    pub num_solved: u32,
    pub num_total: u32,
    pub ac_easy: u32,
    pub ac_medium: u32,
    pub ac_hard: u32,
    pub stat_status_pairs: Vec<QsIndex>,
}

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

#[async_trait::async_trait]
impl InsertToDB for QsIndex {
    type Value = String;
    type Entity = index::Entity;
    type Model = index::Model;
    type ActiveModel = index::ActiveModel;

    fn to_model(&self, category: String) -> Self::Model {
        let pat = serde_json::to_string(self).unwrap_or_default();
        let mut model: index::Model = serde_json::from_str(&pat).unwrap_or_default();
        model.category = category;
        model.pass_rate =
            Some(self.stat.total_acs as f64 / self.stat.total_submitted as f64 * 100.0);
        model.question_id = self.stat.question_id;
        model.question_title = self.stat.question_title.clone();
        model.question_title_slug = self
            .stat
            .question_title_slug
            .clone();
        model.total_acs = self.stat.total_acs;
        model.total_submitted = self.stat.total_submitted;
        model.frontend_question_id = self
            .stat
            .frontend_question_id
            .clone();
        model.difficulty = self.difficulty.level;

        model
    }

    async fn insert_to_db(&mut self, category: String) {
        match Index::insert(
            self.to_model(category)
                .into_active_model(),
        )
        .on_conflict(Self::on_conflict())
        .exec(glob_db())
        .await
        {
            Ok(_) => {}
            Err(err) => error!("{}", err),
        };
    }
    fn on_conflict() -> OnConflict {
        OnConflict::column(index::Column::QuestionId)
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
            .to_owned()
    }
}
