use std::collections::HashSet;

use miette::Result;
use ratatui::widgets::ListState;

use crate::{
    dao::query_topic_tags,
    editor::{edit, CodeTestFile},
    entities::{new_index, topic_tags},
    leetcode::IdSlug,
};

pub struct TopicTagsQS {
    pub topic_tags: Vec<topic_tags::Model>,
    pub topic_tags_state: ListState,

    pub filtered_topic_qs: Vec<new_index::Model>,
    pub filtered_topic_qs_state: ListState,

    pub user_topic_tags: HashSet<String>,
    pub user_topic_tags_translated: HashSet<String>,
    pub user_topic_tags_state: ListState,

    pub filter_index: usize,
}

impl TopicTagsQS {
    pub async fn new() -> Self {
        Self {
            topic_tags: query_topic_tags::query_all_topic()
                .await
                .unwrap_or_default(),
            topic_tags_state: ListState::default(),

            filtered_topic_qs: query_topic_tags::query_by_topic([])
                .await
                .unwrap_or_default(),
            filtered_topic_qs_state: ListState::default(),

            user_topic_tags: HashSet::new(),
            user_topic_tags_translated: HashSet::new(),
            user_topic_tags_state: ListState::default(),

            filter_index: 0,
        }
    }
}

// filtered questions
impl TopicTagsQS {
    pub fn next_topic_qs(&mut self) {
        if self.filtered_topic_qs.is_empty() {
            self.filtered_topic_qs_state
                .select(None);
            return;
        }
        let index = match self
            .filtered_topic_qs_state
            .selected()
        {
            Some(i) => (i + 1) % self.filtered_topic_qs.len(),
            None => 0,
        };
        self.filtered_topic_qs_state
            .select(Some(index));
    }
    pub fn prev_topic_qs(&mut self) {
        if self.filtered_topic_qs.is_empty() {
            self.filtered_topic_qs_state
                .select(None);
            return;
        }
        let index = match self
            .filtered_topic_qs_state
            .selected()
        {
            Some(i) => Some(
                (self.filtered_topic_qs.len() + i - 1) % self.filtered_topic_qs.len(),
            ),
            None => Some(0),
        };
        self.filtered_topic_qs_state
            .select(index);
    }
    pub fn first_topic_qs(&mut self) {
        self.filtered_topic_qs_state
            .select(Some(0));
    }
    pub fn last_topic_qs(&mut self) {
        self.filtered_topic_qs_state
            .select(Some(self.filtered_topic_qs.len() - 1));
    }
    pub fn cur_filtered_qs(&self) -> new_index::Model {
        let index = self
            .filtered_topic_qs_state
            .selected()
            .unwrap_or_default();
        self.filtered_topic_qs
            .get(index)
            .cloned()
            .unwrap_or_default()
    }
    pub async fn confirm_filtered_qs(&mut self) -> Result<()> {
        let qs = self.cur_filtered_qs();
        edit(IdSlug::Slug(qs.title_slug.clone()), CodeTestFile::Code).await
    }
}

// user topic tags
impl TopicTagsQS {
    pub fn prev_user_topic(&mut self) {
        if self.user_topic_tags.is_empty() {
            self.user_topic_tags_state
                .select(None);
            return;
        }
        let index = match self
            .user_topic_tags_state
            .selected()
        {
            Some(i) => {
                Some((self.user_topic_tags.len() + i - 1) % self.user_topic_tags.len())
            }
            None => Some(0),
        };
        self.user_topic_tags_state
            .select(index);
    }

    pub fn next_user_topic(&mut self) {
        if self.user_topic_tags.is_empty() {
            self.user_topic_tags_state
                .select(None);
            return;
        }
        let index = match self
            .user_topic_tags_state
            .selected()
        {
            Some(i) => (i + 1) % self.user_topic_tags.len(),
            None => 0,
        };
        self.user_topic_tags_state
            .select(Some(index));
    }
    pub fn last_user_topic(&mut self) {
        self.user_topic_tags_state
            .select(Some(self.user_topic_tags.len() - 1));
    }
    pub fn first_user_topic(&mut self) {
        self.user_topic_tags_state
            .select(Some(0));
    }
}

// all topic tags, add rm topic
impl TopicTagsQS {
    pub async fn add_or_rm_user_topic(&mut self) {
        let cur_top = self
            .topic_tags_state
            .selected()
            .unwrap_or_default();

        let (topic_slug, translated_slug) = self
            .topic_tags
            .get(cur_top)
            .map(|v| {
                (
                    v.topic_slug.to_owned(),
                    v.name_translated
                        .clone()
                        .unwrap_or_default(),
                )
            })
            .unwrap_or_default();
        if self
            .user_topic_tags
            .contains(&topic_slug)
        {
            self.user_topic_tags
                .remove(&topic_slug);
            self.user_topic_tags_translated
                .remove(&translated_slug);
        } else {
            self.user_topic_tags
                .insert(topic_slug);
            self.user_topic_tags_translated
                .insert(translated_slug);
        }
        self.filtered_topic_qs =
            query_topic_tags::query_by_topic(self.user_topic_tags.clone())
                .await
                .unwrap_or_default();
    }

    // topic_tags //////////////////////////////////
    pub fn first_topic(&mut self) {
        self.topic_tags_state.select(Some(0));
    }
    pub fn last_topic(&mut self) {
        self.topic_tags_state
            .select(Some(self.topic_tags.len() - 1));
    }
    pub fn next_topic(&mut self) {
        if self.topic_tags.is_empty() {
            self.topic_tags_state.select(None);
            return;
        }
        let i = match self.topic_tags_state.selected() {
            Some(i) => (i + 1) % self.topic_tags.len(),
            None => 0,
        };
        self.topic_tags_state.select(Some(i));
    }
    pub fn prev_topic(&mut self) {
        if self.topic_tags.is_empty() {
            self.topic_tags_state.select(None);
            return;
        }
        let i = match self.topic_tags_state.selected() {
            Some(i) => (self.topic_tags.len() + i - 1) % self.topic_tags.len(),
            None => 0,
        };
        self.topic_tags_state.select(Some(i));
    }
}
