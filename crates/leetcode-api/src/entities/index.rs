use std::fmt::Display;

use colored::Colorize;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use unicode_width::UnicodeWidthChar;

use crate::leetcode::question::qs_index::QsIndex;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(DeriveEntityModel)]
#[sea_orm(table_name = "problem_index")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(default)]
    pub question_id: u32,
    #[serde(default)]
    pub question_title: String,
    #[serde(default)]
    pub question_title_slug: String,
    #[serde(default)]
    pub total_acs: u32,
    #[serde(default)]
    pub total_submitted: u32,
    #[serde(default)]
    pub frontend_question_id: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub difficulty: u32,
    #[serde(default)]
    pub paid_only: bool,
    #[serde(default)]
    pub is_favor: bool,
    #[serde(default)]
    pub frequency: u32,
    #[serde(default)]
    pub progress: u32,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub pass_rate: Option<f64>,
}

impl From<QsIndex> for Model {
    fn from(value: QsIndex) -> Self {
        Self {
            question_id: value.stat.question_id,
            question_title: value.stat.question_title,
            question_title_slug: value.stat.question_title_slug,
            total_acs: value.stat.total_acs,
            total_submitted: value.stat.total_submitted,
            frontend_question_id: value.stat.frontend_question_id,
            status: value.status,
            difficulty: value.difficulty.level,
            paid_only: value.paid_only,
            is_favor: value.is_favor,
            frequency: value.frequency,
            progress: value.progress,
            category: String::new(),
            pass_rate: Some(
                value.stat.total_acs as f64 / value.stat.total_submitted as f64 * 100.0,
            ),
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(EnumIter, DeriveRelation)]
pub enum Relation {
    #[default]
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

        let mut id_width = 14;
        for ch in self.frontend_question_id.chars() {
            match UnicodeWidthChar::width(ch).unwrap_or_default() {
                0 | 1 => {},
                w => id_width -= w - 1,
            }
        }

        let mut tit_wid = 66;
        for ch in self.question_title.chars() {
            match UnicodeWidthChar::width(ch).unwrap_or_default() {
                0 | 1 => {},
                w => tit_wid -= w - 1,
            }
        }

        format!(
            "🆔[{id:07}]|{fid:id_width$}|{cg:11}|🇹: {tit:tit_wid$}|Pass: {percent:.2}%|PaidOnly: \
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
