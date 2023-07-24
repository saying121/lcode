use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "problem_index")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub question_id: u64,
    pub question_article_live: Option<bool>,
    pub question_article_slug: Option<String>,
    pub question_article_has_video_solution: Option<bool>,
    pub question_title: String,
    pub question_title_slug: String,
    pub question_hide: bool,
    pub total_acs: u64,
    pub total_submitted: u64,
    pub frontend_question_id: u64,
    pub is_new_question: bool,
    pub status: Option<String>,
    pub difficulty: u64,
    pub paid_only: bool,
    pub is_favor: bool,
    pub frequency: u64,
    pub progress: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::detail::Entity")]
    Detail,
}

impl Related<super::detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Detail.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
