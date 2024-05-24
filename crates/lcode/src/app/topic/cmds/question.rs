use leetcode_api::entities::new_index;
use ratatui::widgets::ListState;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct TopicQuestionState {
    pub all_qs:                  Box<[new_index::Model]>,
    pub filtered_topic_qs_state: ListState,
    pub filtered_qs:             Box<[new_index::Model]>,
}

impl TopicQuestionState {
    pub fn new(all_qs: Box<[new_index::Model]>) -> Self {
        let filtered_qs = all_qs.clone();
        Self {
            all_qs,
            filtered_topic_qs_state: ListState::default(),
            filtered_qs,
        }
    }
    pub fn next(&mut self) {
        let index = self
            .filtered_topic_qs_state
            .selected()
            .map_or(0, |i| (i + 1) % self.filtered_qs.len().max(1));
        self.filtered_topic_qs_state
            .select(Some(index));
    }
    pub fn prev(&mut self) {
        let len = self.filtered_qs.len().max(1);
        let index = self
            .filtered_topic_qs_state
            .selected()
            .map_or(0, |i| (len + i - 1) % len);
        self.filtered_topic_qs_state
            .select(Some(index));
    }
    pub fn last(&mut self) {
        self.filtered_topic_qs_state
            .select(Some(self.filtered_qs.len() - 1));
    }
    pub fn cur_qs_slug(&self) -> Option<String> {
        let index = self
            .filtered_topic_qs_state
            .selected()
            .unwrap_or_default();
        self.filtered_qs
            .get(index)
            .map(|v| v.title_slug.clone())
    }
}
