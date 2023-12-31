use std::fmt::Display;

use colored::Colorize;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use unicode_width::UnicodeWidthChar;

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "problem_index")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(default)]
    pub question_id:          u32,
    #[serde(default)]
    pub question_title:       String,
    #[serde(default)]
    pub question_title_slug:  String,
    #[serde(default)]
    pub total_acs:            u32,
    #[serde(default)]
    pub total_submitted:      u32,
    #[serde(default)]
    pub frontend_question_id: String,
    #[serde(default)]
    pub status:               Option<String>,
    #[serde(default)]
    pub difficulty:           u32,
    #[serde(default)]
    pub paid_only:            bool,
    #[serde(default)]
    pub is_favor:             bool,
    #[serde(default)]
    pub frequency:            u32,
    #[serde(default)]
    pub progress:             u32,
    #[serde(default)]
    pub category:             String,
    #[serde(default)]
    pub pass_rate:            Option<f64>,
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

        let mut widthid = 14;
        let mut count = 0;
        for ch in self.frontend_question_id.chars() {
            if UnicodeWidthChar::width(ch).unwrap_or_default() == 2 {
                count += 1;
            }
        }
        widthid -= count;

        let mut widtit = 66;
        let mut count1 = 0;
        for ch in self.question_title.chars() {
            if UnicodeWidthChar::width(ch).unwrap_or_default() == 2 {
                count1 += 1;
            }
        }
        widtit -= count1;

        format!(
            "🆔[{id:07}]|{fid:widthid$}|{cg:11}|🇹: {tit:widtit$}|Pass: {percent:.2}%|PaidOnly: \
             {po:6}|{diff:8}|{st}",
            fid = self.frontend_question_id,
            id = self.question_id,
            cg = self.category,
            tit = self.question_title,
            percent = self.pass_rate.unwrap_or_default(),
            po = self.paid_only,
            diff = diff,
            st = if self.status.is_some() { "👍" } else { "" }
        )
        .fmt(f)
    }
}
