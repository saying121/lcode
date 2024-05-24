use leetcode_api::entities::index;
use ratatui::widgets::TableState;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct QsState {
    pub all_questions: Box<[index::Model]>,
    pub filtered_qs:   Box<[index::Model]>,
    pub state:         TableState,
}

impl QsState {
    pub fn new(all_questions: Box<[index::Model]>) -> Self {
        let filtered_qs = all_questions.clone();
        Self {
            all_questions,
            filtered_qs,
            state: TableState::default(),
        }
    }
    const fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
    const fn filtered_qs_len(&self) -> usize {
        self.filtered_qs.len()
    }

    pub fn current_qs(&self) -> u32 {
        self.selected().map_or(0, |index| {
            self.filtered_qs
                .get(index)
                .cloned()
                .unwrap_or_default()
                .question_id
        })
    }

    pub fn first(&mut self) -> bool {
        self.state.select(Some(0));
        true
    }
    pub fn last(&mut self) -> bool {
        self.state
            .select(Some(self.filtered_qs_len().saturating_sub(1)));
        true
    }
    pub fn next(&mut self) -> bool {
        let i = self
            .selected()
            .map_or(0, |i| (i + 1) % self.filtered_qs_len().max(1));
        self.state.select(Some(i));
        true
    }
    pub fn prev(&mut self) -> bool {
        let len = self.filtered_qs_len().max(1);
        let i = self
            .state
            .selected()
            .map_or(0, |i| (len + i - 1) % len);
        self.state.select(Some(i));
        true
    }
}
