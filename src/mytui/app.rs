use std::sync::{mpsc::Sender, Arc, Condvar};

use miette::{IntoDiagnostic, Result};
use ratatui::widgets::{ListItem, ListState, ScrollbarState, TableState};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use tui_textarea::TextArea;

use crate::{
    dao::{query_all_index, save_info::CacheFile},
    editor::{edit, CodeTestFile},
    entities::index,
    leetcode::{qs_detail::Question, resps::run_res::RunResult, IdSlug},
};

use super::myevent::UserEvent;

pub struct App<'a> {
    pub questions: Vec<index::Model>,
    pub questions_filtered: Vec<index::Model>,
    pub questions_len: usize,
    pub cur_qs: Question,
    pub state: TableState,

    pub input_line_mode: InputMode,
    pub text_line: TextArea<'a>,

    pub code_block: TextArea<'a>,
    pub edit_code: bool,
    pub code_block_mode: InputMode,

    pub titles: Vec<&'a str>,
    pub tab_index: usize,

    pub tx: Sender<UserEvent>,

    pub sync_state: bool,
    pub sync_title: String,
    pub cur_perc: f64,

    pub vertical_row_len: usize,
    pub horizontal_col_len: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,

    pub submit_res: RunResult,
    pub submiting: bool,
    pub show_submit_res: bool,
    pub test_res: RunResult,
    pub show_test_res: bool,

    pub pop_temp: bool,
    pub temp_str: String,

    pub editor_flag: Arc<std::sync::Mutex<bool>>,
    pub editor_cond: Arc<Condvar>,

    pub save_code: bool,
    pub pop_menu: bool,

    pub l_state: ListState,
    pub l_items: Vec<ListItem<'a>>,

    pub get_count: u32,
}

pub enum InputMode {
    Normal,
    Insert,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl<'a> App<'a> {
    pub async fn new(
        tx: Sender<UserEvent>,
        edit_flag: Arc<std::sync::Mutex<bool>>,
        edit_cond: Arc<Condvar>,
    ) -> App<'a> {
        let questions = query_all_index()
            .await
            .unwrap_or_default();

        Self {
            questions_len: questions.len(),
            questions: questions.clone(),
            questions_filtered: questions,
            cur_qs: Question::default(),
            state: TableState::default(),

            input_line_mode: InputMode::default(),
            text_line: TextArea::default(),

            code_block: TextArea::default(),
            edit_code: false,
            code_block_mode: InputMode::Normal,

            titles: vec!["select question", "edit", "keymaps"],
            tab_index: 0,

            tx,

            sync_state: false,
            sync_title: "".to_owned(),
            cur_perc: 0.0,


            horizontal_col_len: 0,
            horizontal_scroll: 0,
            horizontal_scroll_state: ScrollbarState::default(),
            vertical_row_len: 0,
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),

            submit_res: RunResult::default(),
            submiting: false,
            show_submit_res: false,
            test_res: RunResult::default(),
            show_test_res: false,

            pop_temp: false,
            temp_str: "".to_string(),

            editor_flag: edit_flag,
            editor_cond: edit_cond,

            save_code: false,

            pop_menu: false,

            l_items: vec![
                ListItem::new("Give the project a star, cursor here Press o or Enter"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Global keymap"),
                ListItem::new(""),
                ListItem::new("Shift-Tab/Left   : prev tab"),
                ListItem::new("Tab/Right        : next tab"),
                ListItem::new("Ctrl-q           : exit"),
                ListItem::new("Ctrl-l           : refresh screen"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Tab1/select"),
                ListItem::new(""),
                ListItem::new("j/k              : up/down question"),
                ListItem::new("gg/G             : first/last question"),
                ListItem::new("o                : open with your editor"),
                ListItem::new("Enter            : go to edit tab"),
                ListItem::new("S                : sync question information"),
                ListItem::new("Ctrl-r           : Re get current question"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Tab2/edit"),
                ListItem::new(""),
                ListItem::new("j/k              : Scroll question"),
                ListItem::new("gg/G             : Question content top/end"),
                ListItem::new("Ctrl-p           : Toggle submit menu"),
                ListItem::new("S                : Submit code(just show submit menu)"),
                ListItem::new("T                : Test code(just show submit menu)"),
                ListItem::new("Ctrl-s           : Toggle Submit Result"),
                ListItem::new("Ctrl-t           : Toggle Test Result"),
                ListItem::new("Ctrl-r           : Re get current question"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Tab3/keymaps"),
                ListItem::new(""),
                ListItem::new("j/k              : up/down"),
                ListItem::new("gg               : top"),
                ListItem::new("G                : bottom"),
            ],
            l_state: ListState::default(),

            get_count: 0,
        }
    }

    pub fn first_list(&mut self) {
        self.l_state.select(Some(0));
    }
    pub fn last_list(&mut self) {
        self.l_state
            .select(Some(self.l_items.len() - 1));
    }
    pub fn prev_list(&mut self) {
        let i = match self.l_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.l_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.l_state.select(Some(i));
    }
    pub fn next_list(&mut self) {
        let i = match self.l_state.selected() {
            Some(i) => {
                if i >= self.l_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.l_state.select(Some(i));
    }

    /// from ui to file
    pub async fn save_code(&mut self) -> Result<()> {
        let lines = self
            .code_block
            .clone()
            .into_lines();
        let chf = CacheFile::new(&IdSlug::Id(self.current_qs())).await?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(chf.code_path)
            .await
            .into_diagnostic()?;

        for line in lines {
            file.write_all((line + "\n").as_bytes())
                .await
                .into_diagnostic()?;
        }
        file.sync_all()
            .await
            .into_diagnostic()?;

        Ok(())
    }
    /// from file to ui
    /// # Error:
    /// get qs error (when qs default)
    pub async fn get_code(&mut self, qs: &Question) -> Result<()> {
        if self.cur_qs.question_id != qs.question_id {
            self.code_block = TextArea::default();

            let chf = CacheFile::new(&IdSlug::Id(
                qs.question_id
                    .parse()
                    .into_diagnostic()?,
            ))
            .await?;

            let code = File::open(chf.code_path)
                .await
                .into_diagnostic()?;
            let reader = BufReader::new(code);
            let mut lines = reader.lines();
            while let Some(line) = lines
                .next_line()
                .await
                .into_diagnostic()?
            {
                self.code_block
                    .insert_str(format!("{}", line));
                self.code_block.insert_newline();
            }
            self.code_block.delete_newline();
        }

        Ok(())
    }

    pub fn next_tab(&mut self) -> Result<()> {
        self.tab_index = (self.tab_index + 1) % self.titles.len();
        if self.tab_index == 1 {
            self.tx
                .send(UserEvent::GetQs((self.current_qs(), false)))
                .into_diagnostic()?;
        }
        Ok(())
    }
    pub fn prev_tab(&mut self) -> Result<()> {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.titles.len() - 1;
        }
        if self.tab_index == 1 {
            self.tx
                .send(UserEvent::GetQs((self.current_qs(), false)))
                .into_diagnostic()?;
        }
        Ok(())
    }
    pub fn goto_tab(&mut self, index: usize) -> Result<()> {
        self.tab_index = index;
        if self.tab_index == 1 {
            self.tx
                .send(UserEvent::GetQs((self.current_qs(), false)))
                .into_diagnostic()?;
        }
        Ok(())
    }

    /// next question item
    pub fn next_item(&mut self) {
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

    /// previous question item
    pub fn previous_item(&mut self) {
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

    /// first question item
    pub fn first_item(&mut self) {
        self.state.select(Some(0));
    }
    /// last question item
    pub fn last_item(&mut self) {
        self.state.select(Some(
            self.questions_len
                .saturating_sub(1),
        ));
    }

    /// current selected question id
    pub fn current_qs(&self) -> u32 {
        match self.state.selected() {
            Some(index) => {
                self.questions_filtered
                    .get(index)
                    .map(|v| v.clone())
                    .unwrap_or_default()
                    .question_id
            }
            None => 0,
        }
    }

    /// use outer editor to edit question
    pub async fn confirm(&mut self) -> Result<()> {
        let id = self.current_qs();

        edit(IdSlug::Id(id), CodeTestFile::Code).await
    }
}
