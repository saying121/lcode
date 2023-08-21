use std::sync::mpsc::Sender;

use miette::Result;
use ratatui::widgets::{ScrollbarState, TableState};
use tui_input::Input;

use crate::{
    editor::{edit, CodeTestFile},
    entities::index,
    leetcode::{question_detail::Question, IdSlug},
    storage::query_question::query_all_index,
};

use super::myevent::UserEvent;

pub struct App<'a> {
    pub questions: Vec<index::Model>,
    pub questions_len: usize,
    pub cur_qs: Question,
    pub state: TableState,
    /// Current value of the input box
    pub input: Input,
    /// Current input mode
    pub input_mode: InputMode,

    pub input_code: Input,

    pub titles: Vec<&'a str>,
    pub tab_index: usize,

    pub tx: Sender<UserEvent>,

    pub sync_state: bool,
    pub sync_title: String,
    pub total_index_num: usize,
    pub cur_index_num: usize,

    pub vertical_row_len: usize,
    pub horizontal_col_len: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

pub enum InputMode {
    Normal,
    Editing,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl<'a> App<'a> {
    pub async fn new(tx: Sender<UserEvent>) -> App<'a> {
        let questions = query_all_index()
            .await
            .unwrap_or_default();
        Self {
            input_code: Input::default(),
            questions_len: questions.len(),
            questions,
            cur_qs: Question::default(),
            titles: vec!["select question", "edit"],
            tx,
            sync_state: false,
            sync_title: "".to_owned(),
            state: TableState::default(),
            input: Input::default(),
            input_mode: InputMode::default(),
            tab_index: 0,
            horizontal_col_len: 0,
            horizontal_scroll: 0,
            horizontal_scroll_state: ScrollbarState::default(),
            vertical_row_len: 0,
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            cur_index_num: 0,
            total_index_num: 1,
        }
    }

    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.titles.len();
    }
    pub fn prev_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.titles.len() - 1;
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self
                    .questions_len
                    .saturating_sub(1)
                {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i <= 0 {
                    self.questions_len
                        .saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn first(&mut self) {
        self.state.select(Some(0));
    }
    pub fn last(&mut self) {
        self.state.select(Some(
            self.questions_len
                .saturating_sub(1),
        ));
    }

    pub fn current_qs(&self) -> u32 {
        match self.state.selected() {
            Some(index) => {
                self.questions
                    .get(index)
                    .map(|v| v.clone())
                    .unwrap_or_default()
                    .question_id
            }
            None => 0,
        }
    }

    pub async fn confirm(&mut self) -> Result<()> {
        match self.state.selected() {
            Some(index) => {
                let id = self
                    .questions
                    .get(index)
                    .map(|v| v.clone())
                    .unwrap_or_default();

                edit(IdSlug::Id(id.question_id), CodeTestFile::Code).await
            }
            None => Ok(()),
        }
    }
}
