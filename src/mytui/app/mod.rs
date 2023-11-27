mod tab0;
mod tab1;
mod tab2;
mod tab3;

use std::sync::{mpsc::Sender, Arc, Condvar};

use miette::{IntoDiagnostic, Result};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use tracing::error;
use tui_textarea::TextArea;

use crate::{
    dao::save_info::CacheFile,
    leetcode::{qs_detail::Question, IdSlug},
};

use super::myevent::UserEvent;

#[derive(PartialEq, Eq)]
pub enum Tab2 {
    AllTopics,
    UserTopics,
    Difficulty,
    Questions,
}

pub struct App<'app_lf> {
    pub titles: Vec<&'app_lf str>,
    pub tab_index: usize,

    pub tab0: tab0::SelectQS<'app_lf>,
    pub tab1: tab1::EditCode<'app_lf>,
    pub tab2: tab2::TopicTagsQS<'app_lf>,
    pub tab3: tab3::KeyMaps<'app_lf>,

    pub cur_qs: Question,

    pub tx: Sender<UserEvent>,

    pub pop_temp: bool,
    pub temp_str: String,

    pub editor_flag: Arc<std::sync::Mutex<bool>>,
    pub editor_cond: Arc<Condvar>,

    pub save_code: bool,
}

impl<'app_lf> App<'app_lf> {
    /// stop listen keyevent
    pub fn stop_listen_key(&mut self) {
        *self.editor_flag.lock().unwrap() = false;
    }
    /// start listen keyevent
    pub fn start_listen_key(&mut self) {
        *self.editor_flag.lock().unwrap() = true;
        self.editor_cond.notify_one();
    }

    pub fn test_code(&mut self) -> Result<()> {
        let id = self
            .cur_qs
            .question_id
            .parse()
            .unwrap_or_default();

        self.tx
            .send(UserEvent::TestCode(id))
            .into_diagnostic()?;
        self.tab1.submiting = true;
        Ok(())
    }
    pub fn submit_code(&mut self) -> Result<()> {
        let id: u32 = self
            .cur_qs
            .question_id
            .parse()
            .unwrap_or_default();
        self.tx
            .send(UserEvent::SubmitCode(id))
            .into_diagnostic()?;
        self.tab1.submiting = true;

        Ok(())
    }
}

impl<'app_lf> App<'app_lf> {
    pub fn sync_index(&mut self) -> Result<()> {
        self.tab0.sync_state = true;
        self.tx
            .send(UserEvent::StartSync)
            .into_diagnostic()?;
        Ok(())
    }
    pub fn sync_new(&mut self) -> Result<()> {
        self.tab2.sync_state = true;
        self.tx
            .send(UserEvent::StartSyncNew)
            .into_diagnostic()?;
        Ok(())
    }
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

// tab1 edit
impl<'app_lf> App<'app_lf> {
    /// from ui to file
    pub async fn save_code(&mut self) -> Result<()> {
        let lines = self
            .tab1
            .code_block
            .clone()
            .into_lines();
        let chf = CacheFile::new(&IdSlug::Id(self.tab0.current_qs())).await?;
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
        // if self.cur_qs.question_id != qs.question_id {
        if qs.question_id.is_empty() {
            return Ok(());
        }

        self.tab1.code_block = TextArea::default();

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
            self.tab1
                .code_block
                .insert_str(line);
            self.tab1
                .code_block
                .insert_newline();
        }
        self.tab1
            .code_block
            .delete_newline();

        Ok(())
    }

    pub async fn get_qs_done(&mut self, qs: Question) {
        match self.get_code(&qs).await {
            // if error, don't update question info
            Ok(()) => self.cur_qs = qs,
            Err(err) => error!("{}", err),
        };
    }
}

// base
impl<'app_lf> App<'app_lf> {
    pub async fn new(
        tx: Sender<UserEvent>,
        edit_flag: Arc<std::sync::Mutex<bool>>,
        edit_cond: Arc<Condvar>,
    ) -> App<'app_lf> {
        Self {
            titles: vec!["select question", "edit", "filter with topic", "keymaps"],
            tab_index: 0,

            tx,

            tab0: tab0::SelectQS::new().await,
            tab1: tab1::EditCode::new(),
            tab2: tab2::TopicTagsQS::new().await,
            tab3: tab3::KeyMaps::new(),

            cur_qs: Question::default(),

            pop_temp: false,
            temp_str: String::new(),

            editor_flag: edit_flag,
            editor_cond: edit_cond,

            save_code: false,
        }
    }
    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.titles.len();
    }
    pub fn prev_tab(&mut self) {
        self.tab_index = (self.tab_index + self.titles.len() - 1) % self.titles.len();
    }
    pub fn goto_tab(&mut self, index: usize) -> Result<()> {
        if index == 1 {
            if self.tab_index == 0 {
                self.tx
                    .send(UserEvent::GetQs((
                        IdSlug::Id(self.tab0.current_qs()),
                        false,
                    )))
                    .into_diagnostic()?;
            }
            if self.tab_index == 2 {
                let qs = self.tab2.cur_qs();
                self.tx
                    .send(UserEvent::GetQs((IdSlug::Slug(qs.title_slug), false)))
                    .into_diagnostic()?;
            }
        }
        self.tab_index = index;
        Ok(())
    }
}
