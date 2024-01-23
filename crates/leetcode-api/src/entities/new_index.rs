use std::fmt::Display;

use lcode_config::config::global::G_USER_CONFIG;
use sea_orm::entity::prelude::*;

use crate::leetcode::question::pb_list;

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "new_index")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub title_slug:           String,
    pub title:                String,
    pub title_cn:             Option<String>,
    pub is_favor:             bool,
    pub frontend_question_id: String,
    pub paid_only:            bool,
    pub difficulty:           String,
    pub status:               String,
    #[sea_orm(column_type = "Double", nullable)]
    pub ac_rate:              f64,
    // pub topic_tags: String,
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if G_USER_CONFIG.config.translate {
            let mut name = self
                .title_cn
                .as_deref()
                .unwrap_or_default();
            if name.is_empty() {
                name = self.title.as_str();
            }
            name
        }
        else {
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

impl From<pb_list::NewIndex> for Model {
    fn from(value: pb_list::NewIndex) -> Self {
        Self {
            title_slug:           value.title_slug,
            title:                value.title,
            title_cn:             value.title_cn,
            is_favor:             value.is_favor,
            frontend_question_id: value.frontend_question_id,
            paid_only:            value.paid_only,
            difficulty:           value.difficulty,
            status:               value.status.unwrap_or_default(),
            ac_rate:              value.ac_rate,
        }
    }
}
