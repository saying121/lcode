use std::fmt::Display;

use colored::Colorize;
use sea_orm::entity::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "problem_index")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub question_id: u32,
    pub question_article_live: Option<bool>,
    pub question_article_slug: Option<String>,
    pub question_article_has_video_solution: Option<bool>,
    pub question_title: String,
    pub question_title_slug: String,
    pub question_hide: bool,
    pub total_acs: u32,
    pub total_submitted: u32,
    pub frontend_question_id: String,
    pub is_new_question: bool,
    pub status: Option<String>,
    pub difficulty: u32,
    pub paid_only: bool,
    pub is_favor: bool,
    pub frequency: u32,
    pub progress: u32,
    pub category: String,
    pub pass_rate: Option<f64>,
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

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let diff = match self.difficulty {
            1 => "⛳Easy".green(),
            2 => "🕎Medium".yellow(),
            3 => "💀Hard".red(),
            _ => " Unknown".blue(),
        };

        format!(
            "🆔[{id:07}]|Category: {cg:11}|﫳: {tit:62}|\
            Passing Rate: {percent:.2}%|Paid Only: {po:6}|{diff:8}|",
            id = self.question_id,
            cg = self.category,
            tit = self.question_title,
            percent = self.pass_rate.unwrap_or_default(),
            po = self.paid_only,
            diff = diff
        )
        .fmt(f)
    }
}
