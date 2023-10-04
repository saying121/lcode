use miette::Result;
use ratatui::widgets::TableState;
use tui_textarea::TextArea;

use crate::{
    dao::query_all_index,
    editor::{edit, CodeTestFile},
    entities::index,
    leetcode::{qs_detail::Question, IdSlug},
};

use super::InputMode;

// tab0 select questions
pub struct SelectQS<'tab0> {
    pub questions: Vec<index::Model>,
    pub questions_filtered: Vec<index::Model>,
    pub cur_qs: Question,
    pub state: TableState,

    pub sync_state: bool,
    pub cur_perc: f64,

    pub input_line_mode: InputMode,
    pub text_line: TextArea<'tab0>,
}

impl<'tab0> SelectQS<'tab0> {
    pub async fn new() -> SelectQS<'tab0> {
        let questions = query_all_index()
            .await
            .unwrap_or_default();

        Self {
            questions: questions.clone(),
            questions_filtered: questions,
            cur_qs: Question::default(),
            state: TableState::default(),

            sync_state: false,
            cur_perc: 0.0,

            input_line_mode: InputMode::default(),
            text_line: TextArea::default(),
        }
    }

    /// next question item
    pub fn next_question(&mut self) {
        if self.questions_filtered.is_empty() {
            self.state.select(None);
            return;
        }
        let i = match self.state.selected() {
            Some(i) => Some((i + 1) % self.questions_filtered.len()),
            None => Some(0),
        };
        self.state.select(i);
    }

    /// previous question item
    pub fn previous_question(&mut self) {
        if self.questions_filtered.is_empty() {
            self.state.select(None);
            return;
        }
        let i = match self.state.selected() {
            Some(i) => Some(
                (self.questions_filtered.len() + i - 1) % self.questions_filtered.len(),
            ),
            None => Some(0),
        };
        self.state.select(i);
    }
    /// first question item
    pub fn first_question(&mut self) {
        self.state.select(Some(0));
    }
    /// last question item
    pub fn last_question(&mut self) {
        self.state.select(Some(
            self.questions_filtered
                .len()
                .saturating_sub(1),
        ));
    }

    /// current selected question id
    pub fn current_qs(&self) -> u32 {
        self.state
            .selected()
            .map_or(0, |index| {
                self.questions_filtered
                    .get(index)
                    .cloned()
                    .unwrap_or_default()
                    .question_id
            })
    }

    /// use outer editor to edit question
    pub async fn confirm_qs(&mut self) -> Result<()> {
        let id = self.current_qs();
        // not exists question's id <= 0
        if id < 1 {
            return Ok(());
        }
        edit(IdSlug::Id(id), CodeTestFile::Code).await
    }
}
