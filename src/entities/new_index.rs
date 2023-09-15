use sea_orm::entity::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "new_index")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub title_slug: String,
    pub title: String,
    pub title_cn: Option<String>,
    pub is_favor: Option<bool>,
    pub frontend_question_id: Option<String>,
    pub paid_only: Option<bool>,
    pub difficulty: Option<String>,
    pub status: Option<String>,
    #[sea_orm(column_type = "Double", nullable)]
    pub ac_rate: Option<f64>,
    pub topic_tags: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
