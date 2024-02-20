use sea_orm::{entity::prelude::*, sea_query::OnConflict};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::dao::glob_db;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
#[derive(DeriveEntityModel)]
#[sea_orm(table_name = "topic_tags")]
pub struct Model {
    #[serde(default, alias = "slug")]
    #[sea_orm(primary_key, auto_increment = false)]
    pub topic_slug:      String,
    #[serde(default)]
    pub name:            String,
    #[serde(default, alias = "nameTranslated")]
    pub name_translated: Option<String>,
    #[serde(default)]
    pub id:              String,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::new_index::Entity> for Entity {
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

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct MyTopicTags(Vec<Model>);

impl MyTopicTags {
    pub async fn insert_to_db(self) {
        let temp = self
            .0
            .into_iter()
            .map(sea_orm::IntoActiveModel::into_active_model);
        if let Err(err) = Entity::insert_many(temp)
            .on_conflict(
                OnConflict::column(Column::TopicSlug)
                    .update_columns([
                        Column::Id,
                        Column::TopicSlug,
                        Column::Name,
                        Column::NameTranslated,
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
