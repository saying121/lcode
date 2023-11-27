use sea_orm::{entity::prelude::*, sea_query::OnConflict, IntoActiveModel};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::dao::glob_db;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default, Serialize, Deserialize,
)]
#[sea_orm(table_name = "qs_tag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub title_slug: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub topic_slug: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    TitleSlug,
    TagRelation,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::TitleSlug => Entity::belongs_to(super::new_index::Entity)
                .from(Column::TitleSlug)
                .to(super::new_index::Column::TitleSlug)
                .into(),
            Self::TagRelation => Entity::belongs_to(super::topic_tags::Entity)
                .from(Column::TopicSlug)
                .to(super::topic_tags::Column::TopicSlug)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn inert_to_db(self) {
        let temp = self.into_active_model();

        if let Err(err) = Entity::insert(temp)
            .on_conflict(
                OnConflict::columns([Column::TitleSlug, Column::TopicSlug])
                    .update_columns([Column::TitleSlug, Column::TopicSlug])
                    .to_owned(),
            )
            .exec(glob_db())
            .await
        {
            error!("{}", err);
        }
    }
}
