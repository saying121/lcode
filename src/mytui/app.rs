use std::sync::{mpsc::Sender, Arc, Condvar};

use miette::{IntoDiagnostic, Result};
use ratatui::widgets::{ScrollbarState, TableState};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use tui_textarea::TextArea;

use crate::{
    editor::{edit, CodeTestFile},
    entities::index,
    leetcode::{
        question_detail::Question,
        run_code_resps::{SubmissionDetail, TestResult},
        IdSlug,
    },
    storage::{query_question::query_all_index, Cache},
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
    pub total_index_num: usize,
    pub cur_index_num: usize,

    pub vertical_row_len: usize,
    pub horizontal_col_len: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,

    pub submit_res: SubmissionDetail,
    pub show_submit_res: bool,
    pub test_res: TestResult,
    pub show_test_res: bool,

    pub pop_temp: bool,
    pub temp_str: String,

    pub editor_flag: Arc<std::sync::Mutex<bool>>,
    pub editor_cond: Arc<Condvar>,

    pub save_code: bool,
    pub pop_submit_test: bool,
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

            titles: vec!["select question", "edit"],
            tab_index: 0,

            tx,

            sync_state: false,
            sync_title: "".to_owned(),
            cur_index_num: 0,
            total_index_num: 1,

            horizontal_col_len: 0,
            horizontal_scroll: 0,
            horizontal_scroll_state: ScrollbarState::default(),
            vertical_row_len: 0,
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),

            submit_res: SubmissionDetail::default(),
            show_submit_res: false,
            test_res: TestResult::default(),
            show_test_res: false,

            pop_temp: false,
            temp_str: "".to_string(),

            editor_flag: edit_flag,
            editor_cond: edit_cond,

            save_code: false,

            pop_submit_test: false,
        }
    }
    /// from ui to file
    pub async fn save_code(&mut self) -> Result<()> {
        let lines = self
            .code_block
            .clone()
            .into_lines();
        let (code, _test,_content) =
            Cache::get_code_and_test_path(IdSlug::Id(self.current_qs())).await?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(&code)
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
    pub async fn get_code(&mut self, qs: &Question) -> Result<()> {
        if self.cur_qs.question_id != qs.question_id {
            self.code_block = TextArea::default();

            let (code, _test,_content) = Cache::get_code_and_test_path(IdSlug::Id(
                qs.question_id
                    .parse()
                    .into_diagnostic()?,
            ))
            .await?;

            let code = File::open(code)
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
                .send(UserEvent::GetQs(self.current_qs()))
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
                .send(UserEvent::GetQs(self.current_qs()))
                .into_diagnostic()?;
        }
        Ok(())
    }
    pub fn goto_tab(&mut self, index: usize) -> Result<()> {
        self.tab_index = index;
        if self.tab_index == 1 {
            self.tx
                .send(UserEvent::GetQs(self.current_qs()))
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
