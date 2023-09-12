use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq,Default, Serialize, Deserialize)]
#[sea_orm(table_name = "topic_tags")]
pub struct Model {
    #[serde(default,alias="slug")]
    #[sea_orm(primary_key, auto_increment = false)]
    pub topic_slug: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, alias = "nameTranslated")]
    pub name_translated: Option<String>,
    #[serde(default)]
    pub id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
