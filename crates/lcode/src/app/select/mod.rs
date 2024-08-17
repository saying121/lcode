pub mod cmds;

use crossterm::event::Event as CrossEvent;
use leetcode_api::dao::query::Query;
use rayon::prelude::*;
use tui_textarea::Input;

use self::cmds::{inputline, question, sync_bar};
use crate::fuzzy_search::filter;

// tab0 select questions
#[derive(Clone)]
#[derive(Default)]
#[derive(Debug)]
pub struct SelectQS<'tab0> {
    pub qs_state: question::QsState,
    pub sync_bar: sync_bar::BarState,
    pub inputline: inputline::InputLine<'tab0>,
}

impl<'tab0> SelectQS<'tab0> {
    pub fn keymap_insert(&mut self, event: CrossEvent) -> bool {
        match event.into() {
            Input { key: tui_textarea::Key::Esc, .. } => self.out_edit(),
            Input { key: tui_textarea::Key::Enter, .. } => false, // just one line so do nothing
            input => {
                // Use default key mappings in insert mode(emacs)
                let trigger = self.inputline.handle_input(input);
                if trigger {
                    self.filter_by_input();
                }
                trigger
            },
        }
    }
}

impl<'tab0> SelectQS<'tab0> {
    pub async fn new() -> SelectQS<'tab0> {
        let questions = Query::query_all_index()
            .await
            .unwrap_or_default();

        Self {
            qs_state: question::QsState::new(questions.into()),

            sync_bar: sync_bar::BarState::default(),

            ..Default::default()
        }
    }
    pub fn update_percent(&mut self, perc: f64) {
        self.sync_bar.update(perc);
    }
    /// refresh `filtered_qs`
    pub fn filter_by_input(&mut self) {
        self.qs_state.filtered_qs = self
            .qs_state
            .all_questions
            .par_iter()
            .filter(|v| filter(self.inputline.first_line(), &v.to_string()))
            .cloned()
            .collect();
    }

    /// next question item
    pub fn next_qs(&mut self) -> bool {
        self.qs_state.next()
    }

    /// previous question item
    pub fn prev_qs(&mut self) -> bool {
        self.qs_state.prev()
    }
    /// first question item
    pub fn first_qs(&mut self) -> bool {
        self.qs_state.first()
    }
    /// last question item
    pub fn last_qs(&mut self) -> bool {
        self.qs_state.last()
    }

    /// current selected question id
    pub fn current_qs(&self) -> u32 {
        self.qs_state.current_qs()
    }

    /// enter input line
    pub fn edit(&mut self) -> bool {
        self.inputline.insert();
        true
    }
    pub fn out_edit(&mut self) -> bool {
        self.inputline.out_edit();
        true
    }
}
