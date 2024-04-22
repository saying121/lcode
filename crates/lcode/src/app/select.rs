use crossterm::event::Event as CrossEvent;
use leetcode_api::{dao::query::Query, entities::index};
use ratatui::widgets::TableState;
use rayon::prelude::*;
use tui_textarea::{Input, TextArea};

use super::TuiMode;
use crate::fuzzy_search::filter;

// tab0 select questions
#[derive(Clone)]
#[derive(Default)]
#[derive(Debug)]
pub struct SelectQS<'tab0> {
    pub all_questions: Box<[index::Model]>,
    pub filtered_qs:   Box<[index::Model]>,
    pub state:         TableState,

    pub sync_state: bool,
    pub cur_perc:   f64,

    pub input_line_mode: TuiMode,
    pub text_line:       TextArea<'tab0>,
}

impl<'tab0> SelectQS<'tab0> {
    pub fn keymap_insert(&mut self, event: CrossEvent) -> bool {
        match event.into() {
            Input { key: tui_textarea::Key::Esc, .. } => self.out_edit(),
            Input { key: tui_textarea::Key::Enter, .. } => false,
            input => self.text_line.input(input), // Use default key mappings in insert mode(emacs)
        };
        self.filter_by_input();
        true
    }
}

impl<'tab0> SelectQS<'tab0> {
    pub async fn new() -> SelectQS<'tab0> {
        let questions = Query::query_all_index()
            .await
            .unwrap_or_default();

        Self {
            all_questions: questions.clone().into(),
            filtered_qs: questions.into(),

            sync_state: false,
            cur_perc: 0.0,

            ..Default::default()
        }
    }
    pub fn update_percent(&mut self, cur_perc: f64) {
        self.cur_perc = cur_perc;
    }
    /// refresh `filtered_qs`
    pub fn filter_by_input(&mut self) {
        self.filtered_qs = self
            .all_questions
            .par_iter()
            .filter(|v| filter(&self.text_line.lines()[0], &"", &v.to_string(), 1))
            .cloned()
            .collect();
    }

    /// next question item
    pub fn next_qs(&mut self) -> bool {
        let i = self
            .state
            .selected()
            .map_or(0, |i| (i + 1) % self.filtered_qs.len().max(1));
        self.state.select(Some(i));
        true
    }

    /// previous question item
    pub fn prev_qs(&mut self) -> bool {
        let len = self.filtered_qs.len().max(1);
        let i = self
            .state
            .selected()
            .map_or(0, |i| (len + i - 1) % len);
        self.state.select(Some(i));
        true
    }
    /// first question item
    pub fn first_qs(&mut self) -> bool {
        self.state.select(Some(0));
        true
    }
    /// last question item
    pub fn last_qs(&mut self) -> bool {
        self.state
            .select(Some(self.filtered_qs.len().saturating_sub(1)));
        true
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

    /// enter input line
    pub fn edit(&mut self) -> bool {
        self.input_line_mode = TuiMode::Insert;
        true
    }
    pub fn out_edit(&mut self) -> bool {
        self.input_line_mode = TuiMode::OutEdit;
        true
    }
}
