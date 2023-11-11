use miette::{IntoDiagnostic, Result};
use sea_orm::{sea_query::Expr, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

use crate::entities::{new_index_entity, prelude::*, topic_tags};

use super::glob_db;

pub async fn query_by_topic<I>(topic_slugs: I) -> Result<Vec<new_index_entity::Model>>
where
    I: IntoIterator<Item = String> + Clone,
{
    NewIndexDB::find()
        .distinct()
        .inner_join(topic_tags::Entity)
        .filter(topic_tags::Column::TopicSlug.is_in(topic_slugs))
        .all(glob_db())
        .await
        .into_diagnostic()

    // NewIndexDB::find()
    //     .inner_join(topic_tags::Entity)
    //     .filter(topic_tags::Column::TopicSlug.is_in(topic_slugs.clone()))
    //     .group_by(new_index_entity::Column::TitleSlug)
    //     .expr("COUNT(DISTINCT topic_tags.topic_slug)")
    //     .having(
    //         Expr::col(topic_tags::Column::TopicSlug)
    //             .count()
    //             // .distinct()
    //             .eq(2),
    //     )
    //     .all(glob_db())
    //     .await
    //     .into_diagnostic()
}

pub async fn query_all_topic() -> Result<Vec<topic_tags::Model>> {
    TopicTagsDB::find()
        .all(glob_db())
        .await
        .into_diagnostic()
}

pub async fn query_all_new_index() -> Result<Vec<new_index_entity::Model>> {
    NewIndexDB::find()
        .all(glob_db())
        .await
        .into_diagnostic()
}
