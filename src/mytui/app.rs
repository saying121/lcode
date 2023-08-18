use miette::Result;
use ratatui::widgets::{TableState, ScrollbarState};
use tui_input::Input;

use crate::{
    editor::{edit, CodeTestFile},
    entities::index,
    leetcode::IdSlug,
    storage::query_question::query_all_index,
};

#[derive(Default)]
pub struct App<'a> {
    pub questions: Vec<index::Model>,
    pub state: TableState,
    /// Current value of the input box
    pub input: Input,
    /// Current input mode
    pub input_mode: InputMode,
    pub len: usize,
    pub titles: Vec<&'a str>,
    pub tab_index: usize,

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
    pub async fn new() -> App<'a> {
        let questions = query_all_index()
            .await
            .unwrap_or_default();
        Self {
            len: questions.len(),
            questions,
            titles: vec!["1", "2"],
            ..Default::default()
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
                if i >= self.len - 1 {
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
                    self.len - 1
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
        self.state
            .select(Some(self.len - 1));
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
            None => 1,
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
