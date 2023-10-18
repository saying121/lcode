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
    pub filtered_qs: Vec<index::Model>,
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
            filtered_qs: questions,
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
        let i = self
            .state
            .selected()
            .map_or(0, |i| (i + 1) % self.filtered_qs.len().max(1));
        self.state.select(Some(i));
    }

    /// previous question item
    pub fn previous_question(&mut self) {
        let len = self.filtered_qs.len().max(1);
        let i = self
            .state
            .selected()
            .map_or(0, |i| (len + i - 1) % len);
        self.state.select(Some(i));
    }
    /// first question item
    pub fn first_question(&mut self) {
        self.state.select(Some(0));
    }
    /// last question item
    pub fn last_question(&mut self) {
        self.state.select(Some(
            self.filtered_qs
                .len()
                .saturating_sub(1),
        ));
    }

    /// current selected question id
    pub fn current_qs(&self) -> u32 {
        self.state
            .selected()
            .map_or(0, |index| {
                self.filtered_qs
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
