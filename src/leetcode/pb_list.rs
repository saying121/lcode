use sea_orm::{sea_query::OnConflict, IntoActiveModel};
use serde::{Deserialize, Serialize};

use crate::{
    dao::InsertToDB,
    entities::{new_index, prelude::*, qs_tag, topic_tags},
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

#[async_trait::async_trait]
impl InsertToDB for topic_tags::Model {
    type Value = u32;
    type Entity = TopicTagsDB;
    type Model = Self;
    type ActiveModel = topic_tags::ActiveModel;

    fn on_conflict() -> OnConflict {
        OnConflict::column(topic_tags::Column::TopicSlug)
            .update_columns([
                topic_tags::Column::Id,
                topic_tags::Column::TopicSlug,
                topic_tags::Column::Name,
                topic_tags::Column::NameTranslated,
            ])
            .to_owned()
    }
}
#[async_trait::async_trait]
impl InsertToDB for qs_tag::Model {
    type Value = u32;
    type Entity = QsTagDB;
    type Model = Self;
    type ActiveModel = qs_tag::ActiveModel;

    fn on_conflict() -> OnConflict {
        OnConflict::columns([qs_tag::Column::TopicSlug, qs_tag::Column::TitleSlug])
            .update_columns([qs_tag::Column::TitleSlug, qs_tag::Column::TopicSlug])
            .to_owned()
    }
}
#[async_trait::async_trait]
impl InsertToDB for NewIndex {
    type Value = u32;
    type Entity = NewIndexDB;
    type Model = new_index::Model;
    type ActiveModel = new_index::ActiveModel;

    fn to_model(&self, _v: Self::Value) -> Self::Model {
        let pat = serde_json::to_string(self).unwrap_or_default();
        serde_json::from_str(&pat).unwrap_or_default()
    }
    async fn insert_to_db(&mut self, _v: Self::Value) {
        let topic: Vec<topic_tags::ActiveModel> = self
            .topic_tags
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|v| v.into_active_model())
            .collect();

        let mut qs_tag = Vec::with_capacity(topic.len());

        for i in self
            .topic_tags
            .take()
            .unwrap_or_default()
        {
            let qst = qs_tag::Model {
                topic_slug: i.topic_slug,
                title_slug: self.title_slug.clone(),
            };
            qs_tag.push(qst.into_active_model());
        }

        tokio::join!(
            self.insert_one(_v),
            topic_tags::Model::insert_many(topic),
            qs_tag::Model::insert_many(qs_tag)
        );
    }

    fn on_conflict() -> OnConflict {
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
            .to_owned()
    }
}
