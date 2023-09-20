use sea_orm::{sea_query, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewIndex {
    #[serde(default, alias = "titleSlug")]
    pub title_slug: String,
    #[serde(default)]
    pub title: String,
    #[serde(default, alias = "titleCn")]
    pub title_cn: String,
    #[serde(default, alias = "isFavor")]
    pub is_favor: bool,
    #[serde(default, alias = "frontendQuestionId")]
    pub frontend_question_id: String,
    #[serde(default, alias = "paidOnly")]
    pub paid_only: bool,
    #[serde(default)]
    pub difficulty: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default, alias = "acRate")]
    pub ac_rate: f64,
    #[serde(default, alias = "topicTags")]
    pub topic_tags: Vec<topic_tags::Model>,
}
use crate::entities::prelude::*;
use crate::{
    dao::glob_db,
    entities::{new_index, topic_tags},
};

impl NewIndex {
    pub fn into_active_model(self) -> new_index::ActiveModel {
        let mut tag = String::new();
        for t in self.topic_tags {
            tag += &format!("${}$", t.topic_slug);
        }
        new_index::ActiveModel {
            title_slug: Set(self.title_slug),
            title: Set(self.title),
            title_cn: Set(Some(self.title_cn)),
            ac_rate: Set(Some(self.ac_rate)),
            difficulty: Set(Some(self.difficulty)),
            paid_only: Set(Some(self.paid_only)),
            status: Set(self.status),
            frontend_question_id: Set(Some(self.frontend_question_id)),
            is_favor: Set(Some(self.is_favor)),
            topic_tags: Set(tag),
        }
    }
    pub fn into_active_model_topic(self) -> Vec<topic_tags::ActiveModel> {
        self.topic_tags
            .into_iter()
            .map(|v| v.into())
            .collect()
    }
    pub async fn insert_to_db(self) {
        match NewIndexDB::insert(self.clone().into_active_model())
            .on_conflict(
                sea_query::OnConflict::column(new_index::Column::TitleSlug)
                    .update_columns([
                        new_index::Column::TitleSlug,
                        new_index::Column::Title,
                        new_index::Column::TitleCn,
                        new_index::Column::PaidOnly,
                        new_index::Column::IsFavor,
                        new_index::Column::FrontendQuestionId,
                        new_index::Column::Status,
                        new_index::Column::Difficulty,
                        new_index::Column::AcRate,
                    ])
                    .to_owned(),
            )
            .exec(glob_db())
            .await
        {
            Ok(_) => {}
            Err(err) => error!("{}", err),
        }

        match TopicTagsDB::insert_many(
            self.clone()
                .into_active_model_topic(),
        )
        .on_conflict(
            sea_query::OnConflict::column(topic_tags::Column::TopicSlug)
                .update_columns([
                    topic_tags::Column::Id,
                    topic_tags::Column::TopicSlug,
                    topic_tags::Column::Name,
                    topic_tags::Column::NameTranslated,
                ])
                .to_owned(),
        )
        .exec(glob_db())
        .await
        {
            Ok(_) => {}
            Err(err) => error!("{}", err),
        };
    }
}
