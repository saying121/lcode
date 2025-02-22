use sea_orm::{EntityTrait, IntoActiveModel, sea_query::OnConflict};
use tracing::error;

use super::{pb_list::NewIndex, qs_index::QsIndex};
use crate::{
    dao::{InsertToDB, glob_db},
    entities::{index, new_index, prelude::*, qs_tag, topic_tags},
};

impl InsertToDB for QsIndex {
    type Value = String;
    type Entity = index::Entity;
    type Model = index::Model;
    type ActiveModel = index::ActiveModel;

    fn to_model(&self, category: String) -> Self::Model {
        let mut model: index::Model = self.clone().into();
        model.category = category;
        model
    }

    async fn insert_to_db(&mut self, category: String) {
        match Index::insert(
            self.to_model(category)
                .into_active_model(),
        )
        .on_conflict(Self::on_conflict())
        .exec(glob_db().await)
        .await
        {
            Ok(_) => {},
            Err(err) => error!("{}", err),
        };
    }
    fn on_conflict() -> OnConflict {
        OnConflict::column(index::Column::QuestionId)
            .update_columns([
                index::Column::QuestionTitle,
                index::Column::FrontendQuestionId,
                index::Column::TotalAcs,
                index::Column::TotalSubmitted,
                index::Column::Status,
                index::Column::Difficulty,
                index::Column::PaidOnly,
                index::Column::Frequency,
                index::Column::Progress,
                index::Column::PassRate,
            ])
            .to_owned()
    }
}

impl InsertToDB for topic_tags::Model {
    type Value = u32;
    type Entity = TopicTagsDB;
    type Model = Self;
    type ActiveModel = topic_tags::ActiveModel;

    fn on_conflict() -> OnConflict {
        OnConflict::column(topic_tags::Column::TopicSlug)
            .update_columns([
                topic_tags::Column::Id,
                topic_tags::Column::Name,
                topic_tags::Column::NameTranslated,
            ])
            .to_owned()
    }
}
impl InsertToDB for qs_tag::Model {
    type Value = u32;
    type Entity = QsTagDB;
    type Model = Self;
    type ActiveModel = qs_tag::ActiveModel;

    fn on_conflict() -> OnConflict {
        OnConflict::columns([qs_tag::Column::TopicSlug, qs_tag::Column::TitleSlug])
            .do_nothing()
            .to_owned()
    }
}
impl InsertToDB for NewIndex {
    type Value = u32;
    type Entity = NewIndexDB;
    type Model = new_index::Model;
    type ActiveModel = new_index::ActiveModel;

    fn to_model(&self, _v: Self::Value) -> Self::Model {
        self.clone().into()
    }
    async fn insert_to_db(&mut self, v: Self::Value) {
        let topic: Vec<topic_tags::ActiveModel> = self
            .topic_tags
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(sea_orm::IntoActiveModel::into_active_model)
            .collect();

        let mut qs_tag = Vec::with_capacity(topic.len());

        if let Some(models) = self.topic_tags.take() {
            for ele in models {
                let qst = qs_tag::Model {
                    topic_slug: ele.topic_slug,
                    title_slug: self.title_slug.clone(),
                };
                qs_tag.push(qst.into_active_model());
            }
        }

        tokio::join!(
            self.insert_one(v),
            topic_tags::Model::insert_many(topic),
            qs_tag::Model::insert_many(qs_tag)
        );
    }

    fn on_conflict() -> OnConflict {
        OnConflict::column(new_index::Column::TitleSlug)
            .update_columns([
                new_index::Column::Title,
                new_index::Column::TitleCn,
                new_index::Column::Status,
                new_index::Column::AcRate,
            ])
            .to_owned()
    }
}
