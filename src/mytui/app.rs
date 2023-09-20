use std::{
    collections::HashSet,
    sync::{mpsc::Sender, Arc, Condvar},
};

use miette::{IntoDiagnostic, Result};
use ratatui::widgets::{ListItem, ListState, ScrollbarState, TableState};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use tui_textarea::TextArea;

use crate::{
    dao::{query_all_index, query_topic_tags, save_info::CacheFile},
    editor::{edit, CodeTestFile},
    entities::{index, new_index, topic_tags},
    leetcode::{qs_detail::Question, resps::run_res::RunResult, IdSlug},
};

use super::myevent::UserEvent;

pub struct App<'app_lf> {
    pub questions: Vec<index::Model>,
    pub questions_filtered: Vec<index::Model>,
    pub questions_len: usize,
    pub cur_qs: Question,
    pub state: TableState,

    pub input_line_mode: InputMode,
    pub text_line: TextArea<'app_lf>,

    pub code_block: TextArea<'app_lf>,
    pub edit_code: bool,
    pub code_block_mode: InputMode,

    pub titles: Vec<&'app_lf str>,
    pub tab_index: usize,

    pub tx: Sender<UserEvent>,

    pub sync_state: bool,
    pub sync_title: String,
    pub cur_perc: f64,

    pub vertical_row_len: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_col_len: usize,
    pub horizontal_scroll_state: ScrollbarState,
    pub horizontal_scroll: usize,

    // test and submit
    pub submiting: bool,

    pub submit_res: RunResult,
    pub show_submit_res: bool,
    pub submit_vert_scroll_state: ScrollbarState,
    pub submit_vert_scroll: usize,
    pub submit_hori_scroll_state: ScrollbarState,
    pub submit_hori_scroll: usize,
    pub submit_row_len: usize,

    pub test_res: RunResult,
    pub show_test_res: bool,
    pub test_vert_scroll_state: ScrollbarState,
    pub test_vert_scroll: usize,
    pub test_hori_scroll_state: ScrollbarState,
    pub test_hori_scroll: usize,
    pub test_row_len: usize,

    pub pop_temp: bool,
    pub temp_str: String,

    pub editor_flag: Arc<std::sync::Mutex<bool>>,
    pub editor_cond: Arc<Condvar>,

    pub save_code: bool,
    pub show_pop_menu: bool,

    pub l_state: ListState,
    pub l_items: Vec<ListItem<'app_lf>>,

    pub get_count: u32,

    pub topic_tags: Vec<topic_tags::Model>,
    pub topic_state: ListState,
    pub filtered_topic_qs: Vec<new_index::Model>,
    pub filtered_topic_qs_state: ListState,

    pub user_topic_tags: HashSet<String>,
    pub user_topic_tags_state: ListState,

    pub filter_index: usize,
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

impl<'app_lf> App<'app_lf> {
    pub async fn new(
        tx: Sender<UserEvent>,
        edit_flag: Arc<std::sync::Mutex<bool>>,
        edit_cond: Arc<Condvar>,
    ) -> App<'app_lf> {
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

            titles: vec!["select question", "edit", "filter with topic", "keymaps"],
            tab_index: 0,

            tx,

            sync_state: false,
            sync_title: String::new(),
            cur_perc: 0.0,

            horizontal_col_len: 0,
            horizontal_scroll: 0,
            horizontal_scroll_state: ScrollbarState::default(),
            vertical_row_len: 0,
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),

            // submit and test
            submiting: false,

            submit_res: RunResult::default(),
            show_submit_res: false,
            submit_vert_scroll_state: ScrollbarState::default(),
            submit_vert_scroll: 0,
            submit_hori_scroll_state:ScrollbarState::default(),
            submit_hori_scroll: 0,
            submit_row_len: 0,

            test_res: RunResult::default(),
            show_test_res: false,
            test_vert_scroll_state: ScrollbarState::default(),
            test_vert_scroll: 0,
            test_hori_scroll_state:ScrollbarState::default(),
            test_hori_scroll:0,
            test_row_len: 0,

            pop_temp: false,
            temp_str: String::new(),

            editor_flag: edit_flag,
            editor_cond: edit_cond,

            save_code: false,

            show_pop_menu: false,

            l_items: vec![
                ListItem::new("Give the project a star, cursor here Press o or Enter"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Global keymap"),
                ListItem::new(""),
                ListItem::new("Shift-Tab/Left   : Prev tab"),
                ListItem::new("Tab/Right        : Next tab"),
                ListItem::new("Ctrl-q           : Exit"),
                ListItem::new("Ctrl-l           : Refresh screen"),
                ListItem::new("gg/G             : Top/Bottom"),
                ListItem::new("j/k              : Up/Down"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Tab1/select"),
                ListItem::new(""),
                ListItem::new("o                : Open with your editor"),
                ListItem::new("C                : Edit config"),
                ListItem::new("Enter            : Go to edit tab"),
                ListItem::new("S                : Sync question information"),
                ListItem::new("Ctrl-r           : Re get current question"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Tab2/edit"),
                ListItem::new(""),
                ListItem::new("Ctrl-p           : Toggle submit menu"),
                ListItem::new("S                : Submit code(just show submit menu)"),
                ListItem::new("T                : Test code(just show submit menu)"),
                ListItem::new("Ctrl-s           : Toggle Submit Result"),
                ListItem::new("Ctrl-t           : Toggle Test Result"),
                ListItem::new("Ctrl-r           : Re get current question, notice it will reget question by tab1 info"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Tab3/filter with topic"),
                ListItem::new(""),
                ListItem::new("Ctrl-l           : Go to right"),
                ListItem::new("Ctrl-h           : Go to left"),
                ListItem::new("Ctrl-k           : Go to up"),
                ListItem::new("Ctrl-j           : Go to down"),
                ListItem::new("Enter(all topic) : Toggle topic"),
                ListItem::new("Enter(questions) : Confirm"),
                // ListItem::new("S                : Sync db"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Tab4/keymaps"),
                ListItem::new(""),
            ],
            l_state: ListState::default(),

            get_count: 0,

            topic_tags: query_topic_tags::query_all_topic()
                .await
                .unwrap_or_default(),
            topic_state: ListState::default(),

            filtered_topic_qs: query_topic_tags::query_by_topic([])
                .await
                .unwrap_or_default(),
            filtered_topic_qs_state: ListState::default(),

            user_topic_tags: HashSet::new(),
            user_topic_tags_state: ListState::default(),

            filter_index: 0,
        }
    }

    pub async fn add_or_rm_user_topic(&mut self) {
        let cur_top = self
            .topic_state
            .selected()
            .unwrap_or_default();

        let topic_slug = self
            .topic_tags
            .get(cur_top)
            .map(|v| v.topic_slug.to_owned())
            .unwrap_or_default();
        if self
            .user_topic_tags
            .contains(&topic_slug)
        {
            self.user_topic_tags
                .remove(&topic_slug);
        } else {
            self.user_topic_tags
                .insert(topic_slug);
        }
        self.filtered_topic_qs =
            query_topic_tags::query_by_topic(self.user_topic_tags.clone())
                .await
                .unwrap_or_default();
    }

    ////////////////////////////////////
    pub fn first_topic(&mut self) {
        self.topic_state.select(Some(0));
    }
    pub fn last_topic(&mut self) {
        self.topic_state
            .select(Some(self.topic_tags.len() - 1));
    }
    pub fn next_topic(&mut self) {
        let i = match self.topic_state.selected() {
            Some(i) => {
                if i >= self.topic_tags.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.topic_state.select(Some(i));
    }
    pub fn prev_topic(&mut self) {
        let i = match self.topic_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.topic_tags.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.topic_state.select(Some(i));
    }
    ////////////////////////////////////
    pub fn next_topic_qs(&mut self) {
        let i = match self
            .filtered_topic_qs_state
            .selected()
        {
            Some(i) => {
                if i >= self.filtered_topic_qs.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.filtered_topic_qs_state
            .select(Some(i));
    }
    pub fn prev_user_topic(&mut self) {
        let i = match self
            .user_topic_tags_state
            .selected()
        {
            Some(i) => {
                if i == 0 {
                    self.user_topic_tags.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.user_topic_tags_state
            .select(Some(i));
    }
    pub fn last_user_topic(&mut self) {
        self.user_topic_tags_state
            .select(Some(self.user_topic_tags.len() - 1));
    }
    pub fn first_user_topic(&mut self) {
        self.user_topic_tags_state
            .select(Some(0));
    }
    pub fn cur_filtered_qs(&self) -> new_index::Model {
        let index = self
            .filtered_topic_qs_state
            .selected()
            .unwrap_or_default();
        self.filtered_topic_qs
            .get(index)
            .cloned()
            .unwrap_or_default()
    }
    pub fn confirm_filtered_qs(&mut self) {
        self.goto_tab(1)
            .unwrap_or_default();
    }
    ////////////////////////////////////

    pub fn next_user_topic(&mut self) {
        let i = match self
            .user_topic_tags_state
            .selected()
        {
            Some(i) => {
                if i >= self.user_topic_tags.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.user_topic_tags_state
            .select(Some(i));
    }
    pub fn prev_topic_qs(&mut self) {
        let i = match self
            .filtered_topic_qs_state
            .selected()
        {
            Some(i) => {
                if i == 0 {
                    self.filtered_topic_qs.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.filtered_topic_qs_state
            .select(Some(i));
    }
    pub fn first_topic_qs(&mut self) {
        self.filtered_topic_qs_state
            .select(Some(0));
    }
    pub fn last_topic_qs(&mut self) {
        self.filtered_topic_qs_state
            .select(Some(self.filtered_topic_qs.len() - 1));
    }
    ////////////////////////////////////

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
    ////////////////////////////////////

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
                self.code_block.insert_str(line);
                self.code_block.insert_newline();
            }
            self.code_block.delete_newline();
        }

        Ok(())
    }

    pub fn next_tab(&mut self) -> Result<()> {
        self.tab_index = (self.tab_index + 1) % self.titles.len();
        // if self.tab_index == 1 {
        //     self.tx
        //         .send(UserEvent::GetQs((IdSlug::Id(self.current_qs()), false)))
        //         .into_diagnostic()?;
        // }
        Ok(())
    }
    pub fn prev_tab(&mut self) -> Result<()> {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.titles.len() - 1;
        }
        // if self.tab_index == 1 {
        //     self.tx
        //         .send(UserEvent::GetQs((IdSlug::Id(self.current_qs()), false)))
        //         .into_diagnostic()?;
        // }
        Ok(())
    }
    pub fn goto_tab(&mut self, index: usize) -> Result<()> {
        if index == 1 {
            if self.tab_index == 0 {
                self.tx
                    .send(UserEvent::GetQs((IdSlug::Id(self.current_qs()), false)))
                    .into_diagnostic()?;
            }
            if self.tab_index == 2 {
                let qs = self.cur_filtered_qs();
                self.tx
                    .send(UserEvent::GetQs((IdSlug::Slug(qs.title_slug), false)))
                    .into_diagnostic()?;
            }
        }
        self.tab_index = index;
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
                if i == 0 {
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
                    .cloned()
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
