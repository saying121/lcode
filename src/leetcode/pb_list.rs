use sea_orm::{sea_query::OnConflict, EntityTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::entities::{prelude::*, qs_tag};
use crate::{
    dao::glob_db,
    entities::{new_index, topic_tags},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Data {
    #[serde(default)]
    pub data: DataInner,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DataInner {
    #[serde(default, alias = "problemsetQuestionList")]
    pub problemset_question_list: ProblemsetQuestionList,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProblemsetQuestionList {
    #[serde(default, alias = "hasMore")]
    pub has_more: bool, // for leetcode.cn
    #[serde(default)]
    pub total: u32,
    #[serde(default)]
    pub questions: Vec<NewIndex>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewIndex {
    #[serde(default, alias = "titleSlug")]
    pub title_slug: String,
    #[serde(default)]
    pub title: String,
    #[serde(default, alias = "titleCn")]
    pub title_cn: Option<String>,
    #[serde(default, alias = "isFavor")]
    pub is_favor: bool,
    #[serde(default, alias = "frontendQuestionId")]
    pub frontend_question_id: String,
    #[serde(default, alias = "paidOnly")]
    pub paid_only: bool,
    #[serde(default)]
    pub difficulty: String,
    #[serde(default)]
    pub status: String,
    #[serde(default, alias = "acRate")]
    pub ac_rate: f64,
    #[serde(default, alias = "topicTags")]
    pub topic_tags: Option<Vec<topic_tags::Model>>,
}

impl NewIndex {
    fn into_model(
        mut self,
    ) -> (
        Vec<topic_tags::ActiveModel>,
        new_index::ActiveModel,
        Vec<qs_tag::ActiveModel>,
    ) {
        let topic_model = self
            .topic_tags
            .clone()
            .unwrap_or_default();
        let mut qs_tag = Vec::with_capacity(topic_model.len());

        for i in topic_model {
            let qst = qs_tag::Model {
                topic_slug: i.topic_slug,
                title_slug: self.title_slug.clone(),
            };
            qs_tag.push(qst.into_active_model());
        }

        let tag = self
            .topic_tags
            .take()
            .unwrap_or_default()
            .into_iter()
            .map(|v| v.into())
            .collect();
        let new_index: new_index::Model = self.into();

        (tag, new_index.into_active_model(), qs_tag)
    }

    pub async fn insert_to_db(self) {
        let (topic, index, qs_tag) = self.into_model();

        let new_index = NewIndexDB::insert(index)
            .on_conflict(
                OnConflict::column(new_index::Column::TitleSlug)
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
            .exec(glob_db());

        let topic_tag = TopicTagsDB::insert_many(topic)
            .on_conflict(
                OnConflict::column(topic_tags::Column::TopicSlug)
                    .update_columns([
                        topic_tags::Column::Id,
                        topic_tags::Column::TopicSlug,
                        topic_tags::Column::Name,
                        topic_tags::Column::NameTranslated,
                    ])
                    .to_owned(),
            )
            .exec(glob_db());

        let qs_tag = QsTagDB::insert_many(qs_tag)
            .on_conflict(
                OnConflict::columns([
                    qs_tag::Column::TopicSlug,
                    qs_tag::Column::TitleSlug,
                ])
                .update_columns([qs_tag::Column::TitleSlug, qs_tag::Column::TopicSlug])
                .to_owned(),
            )
            .exec(glob_db());

        let (a, b) = tokio::join!(topic_tag, new_index);

        if let Err(err) = a {
            error!("{}", err);
        }
        if let Err(err) = b {
            error!("{}", err);
        }
        if let Err(err) = qs_tag.await {
            error!("{}", err);
        }
    }
}
