use miette::{IntoDiagnostic, Result};
use sea_orm::{sea_query::Expr, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

use crate::entities::{new_index, prelude::*, topic_tags};

use super::glob_db;

pub async fn query_by_topic(
    topic_slugs: &[String],
    diff: Option<String>,
) -> Result<Vec<new_index::Model>> {
    let mut cond = topic_tags::Column::TopicSlug.is_in(topic_slugs);

    if let Some(v) = diff {
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
        .all(glob_db())
        .await
        .into_diagnostic()
}

pub async fn query_status() -> Result<Vec<(String, u32, u32)>> {
    use sea_orm::{DeriveColumn, EnumIter};

    #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
    enum QueryAs {
        Diff,
        PassCount,
        Sum,
    }

    NewIndexDB::find()
        .select_only()
        .column_as(new_index::Column::Difficulty, QueryAs::Diff)
        .column_as(
            Expr::expr(new_index::Column::Status.eq("AC")).sum(),
            QueryAs::PassCount,
        )
        .column_as(new_index::Column::TitleSlug.count(), QueryAs::Sum)
        .group_by(new_index::Column::Difficulty)
        .into_values::<(String, u32, u32), QueryAs>()
        .all(glob_db())
        .await
        .into_diagnostic()
}

pub async fn query_all_topic() -> Result<Vec<topic_tags::Model>> {
    TopicTagsDB::find()
        .all(glob_db())
        .await
        .into_diagnostic()
}

pub async fn query_all_new_index(diff: Option<String>) -> Result<Vec<new_index::Model>> {
    if let Some(diff) = diff {
        if !diff.is_empty() {
            return NewIndexDB::find()
                .filter(new_index::Column::Difficulty.eq(diff))
                .all(glob_db())
                .await
                .into_diagnostic();
        }
    }
    NewIndexDB::find()
        .all(glob_db())
        .await
        .into_diagnostic()
}
