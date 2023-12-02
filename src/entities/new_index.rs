use std::fmt::Display;

use sea_orm::{entity::prelude::*, sea_query::OnConflict, IntoActiveModel};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{config::global::glob_user_config, dao::glob_db};

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "new_index")]
pub struct Model {
    #[serde(default, alias = "titleSlug")]
    #[sea_orm(primary_key, auto_increment = false)]
    pub title_slug: String,
    #[serde(default)]
    pub title: String,
    #[serde(default, alias = "titleCn")]
    pub title_cn: Option<String>,
    #[serde(default, alias = "isFavor")]
    pub is_favor: bool,
    #[serde(default, alias = "frontendQuestionId")]
    pub frontend_question_id: String,
    #[serde(default, alias = "paidOnly")]
    pub paid_only: bool,
    #[serde(default)]
    pub difficulty: String,
    #[serde(default)]
    pub status: String,
    #[serde(default, alias = "acRate")]
    #[sea_orm(column_type = "Double", nullable)]
    pub ac_rate: f64,
    // pub topic_tags: String,
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if glob_user_config().translate {
            let mut name = self
                .title_cn
                .as_deref()
                .unwrap_or_default();
            if name.is_empty() {
                name = self.title.as_str();
            }
            name
        } else {
            self.title.as_str()
        };

        format!(
            "{id}: {tit}, {st}",
            id = self.frontend_question_id,
            tit = name,
            st = self.status
        )
        .fmt(f)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::topic_tags::Entity> for Entity {
    fn to() -> RelationDef {
        super::qs_tag::Relation::TagRelation.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::qs_tag::Relation::TitleSlug
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<crate::leetcode::pb_list::NewIndex> for Model {
    fn from(value: crate::leetcode::pb_list::NewIndex) -> Self {
        Self {
            title_slug: value.title_slug,
            title: value.title,
            title_cn: value.title_cn,
            is_favor: value.is_favor,
            frontend_question_id: value.frontend_question_id,
            paid_only: value.paid_only,
            difficulty: value.difficulty,
            status: value.status,
            ac_rate: value.ac_rate,
        }
    }
}

impl Model {
    pub async fn insert_to_db(self) {
        if let Err(err) = Entity::insert(self.into_active_model())
            .on_conflict(
                OnConflict::column(Column::TitleSlug)
                    .update_columns([
                        Column::TitleSlug,
                        Column::Title,
                        Column::TitleCn,
                        Column::PaidOnly,
                        Column::IsFavor,
                        Column::FrontendQuestionId,
                        Column::Status,
                        Column::Difficulty,
                        Column::AcRate,
                    ])
                    .to_owned(),
            )
            .exec(glob_db().await)
            .await
        {
            error!("{}", err);
        }
    }
}
