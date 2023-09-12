use miette::{IntoDiagnostic, Result};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::entities::prelude::*;
use crate::entities::{new_index, topic_tags};

use super::glob_db;

pub async fn find_topic(topic_slugs: Vec<&str>) -> Result<Vec<new_index::Model>> {
    let mut cond = Condition::all();
    for t_slug in topic_slugs {
        cond = cond.add(new_index::Column::TopicTags.contains(format!("${}$", t_slug)));
    }

    let res: Vec<new_index::Model> = NewIndexDB::find()
        .filter(cond)
        .all(glob_db())
        .await
        .into_diagnostic()?;
    Ok(res)
}
pub async fn get_all_topic() -> Result<Vec<topic_tags::Model>> {
    TopicTagsDB::find()
        .all(glob_db())
        .await
        .into_diagnostic()
}
