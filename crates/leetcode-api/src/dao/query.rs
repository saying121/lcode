use miette::{IntoDiagnostic, Result};
use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect, sea_query::Expr,
};
use tracing::debug;

use super::{detail, glob_db, index};
use crate::{
    entities::{new_index, prelude::*, topic_tags},
    leetcode::IdSlug,
};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Query;

#[derive(FromQueryResult)]
#[derive(Debug)]
#[derive(Clone)]
pub struct PassStat {
    pub diff: String,
    pub pass_count: u32,
    pub sum: u32,
}

impl Query {
    /// Find the problem, return one
    ///
    /// * `idslug`: id or title
    pub async fn get_question_index(idslug: &IdSlug) -> Result<index::Model> {
        let res = match idslug {
            IdSlug::Id(id) => Index::find_by_id(*id)
                .one(glob_db().await)
                .await
                .into_diagnostic()?
                .unwrap_or_default(),
            IdSlug::Slug(slug) => Index::find()
                .filter(index::Column::QuestionTitleSlug.eq(slug))
                .one(glob_db().await)
                .await
                .into_diagnostic()?
                .unwrap_or_default(),
        };
        debug!("get value {:#?}", res);
        Ok(res)
    }

    pub async fn query_detail_by_id(id: u32) -> Result<Option<detail::Model>> {
        Detail::find_by_id(id)
            .one(glob_db().await)
            .await
            .into_diagnostic()
    }

    pub async fn query_all_index() -> Result<Vec<index::Model>> {
        let models = Index::find()
            .all(glob_db().await)
            .await
            .into_diagnostic()?;

        Ok(models)
    }
    pub async fn query_by_topic(
        topic_slugs: &[String],
        difficulty: Option<String>,
    ) -> Result<Vec<new_index::Model>> {
        let mut cond = topic_tags::Column::TopicSlug.is_in(topic_slugs);

        if let Some(v) = difficulty {
            if !v.is_empty() {
                cond = cond.and(new_index::Column::Difficulty.eq(v));
            }
        }

        NewIndexDB::find()
            .inner_join(topic_tags::Entity)
            .filter(cond)
            .group_by(new_index::Column::TitleSlug)
            .having(
                topic_tags::Column::TopicSlug
                    .count()
                    .eq(topic_slugs.len() as i32),
            )
            .all(glob_db().await)
            .await
            .into_diagnostic()
    }

    pub async fn query_status() -> Result<Vec<PassStat>> {
        NewIndexDB::find()
            .select_only()
            .column_as(new_index::Column::Difficulty, "diff")
            .column_as(
                Expr::expr(new_index::Column::Status.eq("AC")).sum(),
                "pass_count",
            )
            .column_as(new_index::Column::TitleSlug.count(), "sum")
            .group_by(new_index::Column::Difficulty)
            .into_model::<PassStat>()
            .all(glob_db().await)
            .await
            .into_diagnostic()
    }

    pub async fn query_all_topic() -> Result<Vec<topic_tags::Model>> {
        TopicTagsDB::find()
            .all(glob_db().await)
            .await
            .into_diagnostic()
    }

    pub async fn query_all_new_index<D>(diff: D) -> Result<Vec<new_index::Model>>
    where
        D: Into<Option<String>> + Send,
    {
        if let Some(diff) = diff.into() {
            if !diff.is_empty() {
                return NewIndexDB::find()
                    .filter(new_index::Column::Difficulty.eq(diff))
                    .all(glob_db().await)
                    .await
                    .into_diagnostic();
            }
        }
        NewIndexDB::find()
            .all(glob_db().await)
            .await
            .into_diagnostic()
    }
}
