use leetcode_api::entities::topic_tags;
use ratatui::widgets::ListState;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct TopicsState {
    pub topic_tags:       Box<[topic_tags::Model]>,
    pub topic_tags_state: ListState,

    pub user_topic_tags:            Vec<String>,
    pub user_topic_tags_translated: Vec<String>,
    pub user_topic_tags_state:      ListState,
}

impl TopicsState {
    /// if remove a topic return `true` or `false`
    pub fn rm_user_topic(&mut self) -> bool {
        let mut trigger = false;
        let cur_top = self
            .user_topic_tags_state
            .selected()
            .unwrap_or_default();

        if !self.user_topic_tags.is_empty() {
            self.user_topic_tags.remove(cur_top);
            self.user_topic_tags_translated
                .remove(cur_top);
            trigger = true;
        }
        if cur_top >= self.user_topic_tags.len() {
            self.prev_user();
        }
        trigger
    }
    /// if add a topic return `true` or `false`
    pub fn add_user_topic(&mut self) -> bool {
        let mut trigger = false;
        let cur_top = self
            .topic_tags_state
            .selected()
            .unwrap_or_default();

        let (topic_slug, translated_slug) = self
            .topic_tags
            .get(cur_top)
            .map(|v| {
                (
                    v.topic_slug.clone(),
                    v.name_translated
                        .clone()
                        .unwrap_or_default(),
                )
            })
            .unwrap_or_default();

        if !self
            .user_topic_tags
            .contains(&topic_slug)
        {
            self.user_topic_tags_translated.push(
                if translated_slug.is_empty() {
                    topic_slug.clone()
                }
                else {
                    translated_slug
                },
            );
            self.user_topic_tags.push(topic_slug);
            trigger = true;
        }
        trigger
    }
}

impl TopicsState {
    pub fn prev_user(&mut self) {
        let index = self
            .user_topic_tags_state
            .selected()
            .map_or(0, |i| i.saturating_sub(1));
        self.user_topic_tags_state
            .select(Some(index));
    }
    pub fn next_user(&mut self) {
        let index = self
            .user_topic_tags_state
            .selected()
            .map_or(0, |i| {
                i.saturating_add(1).min(
                    self.user_topic_tags
                        .len()
                        .saturating_sub(1),
                )
            });
        self.user_topic_tags_state
            .select(Some(index));
    }
    pub fn last_user(&mut self) {
        self.user_topic_tags_state
            .select(Some(self.user_topic_tags.len() - 1));
    }
    pub fn first_user(&mut self) {
        self.user_topic_tags_state
            .select(Some(0));
    }
}

impl TopicsState {
    pub fn new(topic_tags: Box<[topic_tags::Model]>) -> Self {
        Self {
            topic_tags,
            topic_tags_state: ListState::default(),

            user_topic_tags: vec![],
            user_topic_tags_translated: vec![],
            user_topic_tags_state: ListState::default(),
        }
    }
    pub fn first(&mut self) {
        self.topic_tags_state.select(Some(0));
    }
    pub fn last(&mut self) {
        self.topic_tags_state
            .select(Some(self.topic_tags.len() - 1));
    }
    pub fn next(&mut self) {
        let i = self
            .topic_tags_state
            .selected()
            .map_or(0, |i| {
                i.saturating_add(1)
                    .min(self.topic_tags.len().saturating_sub(1))
            });
        self.topic_tags_state.select(Some(i));
    }
    pub fn prev(&mut self) {
        let i = self
            .topic_tags_state
            .selected()
            .map_or(0, |i| i.saturating_sub(1));
        self.topic_tags_state.select(Some(i));
    }
}
