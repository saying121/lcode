use leetcode_api::{
    dao::{get_question_index, save_info::CacheFile},
    leetcode::{
        question::qs_detail::Question,
        resps::{checkin::TotalPoints, user_data::UserStatus},
        IdSlug,
    },
};
use miette::{IntoDiagnostic, Result};
use tokio::{
    self,
    fs::{File, OpenOptions},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use tui_textarea::TextArea;

use super::{dispatch::next_key, edit::EditCode, infos, select, topic, TuiIndex};
use crate::{
    editor::{self, CodeTestFile},
    glob_leetcode,
    mytui::myevent::EventsHandler,
};

#[derive(Debug)]
#[derive(Default)]
pub struct App<'app> {
    pub titles:    Vec<&'app str>,
    pub tab_index: TuiIndex,

    pub select: select::SelectQS<'app>,
    pub edit:   EditCode<'app>,
    pub topic:  topic::TopicTagsQS<'app>,
    pub infos:  infos::KeyMaps<'app>,

    pub cur_qs: Question,

    pub pop_temp: bool,
    pub temp_str: String,

    pub save_code: bool,

    pub next_key: next_key::NextKey,

    pub events: EventsHandler,

    pub user_status: UserStatus,
    pub points:      TotalPoints,
}

impl<'app_lf> App<'app_lf> {
    /// edit cursor qs with outer editor, for select tab
    pub async fn select_edit_cur_qs(&mut self) -> Result<()> {
        let id = self.select.current_qs();
        // not exists question's id <= 0
        if id < 1 {
            return Ok(());
        }
        self.pause();
        editor::open(IdSlug::Id(id), CodeTestFile::Code).await?;
        self.r#continue();
        Ok(())
    }
    /// edit cursor qs with outer editor, for edit tab
    pub(crate) async fn edit_tab_edit_with_editor(&mut self) -> Result<()> {
        let qs_slug = self
            .cur_qs
            .qs_slug
            .clone()
            .unwrap_or_default();
        if qs_slug.is_empty() {
            return Ok(());
        }
        self.pause();
        editor::open(IdSlug::Slug(qs_slug), CodeTestFile::Code).await?;
        self.r#continue();

        self.get_code(&self.cur_qs.clone())
            .await?;

        Ok(())
    }
    /// edit cursor qs with outer editor, for topic tab
    pub async fn topic_edit_cur_qs(&mut self) -> Result<()> {
        let qs_slug = self.topic.cur_qs_slug();
        if let Some(slug) = qs_slug {
            self.pause();
            editor::open(IdSlug::Slug(slug), CodeTestFile::Code).await?;
            self.r#continue();
        }
        Ok(())
    }
}

// tab1 edit
impl<'app_lf> App<'app_lf> {
    /// from ui to file
    pub async fn save_code(&mut self) -> Result<()> {
        self.save_code = true;

        let pb = get_question_index(&IdSlug::Slug(
            self.cur_qs
                .qs_slug
                .clone()
                .unwrap_or_default(),
        ))
        .await?;
        let chf = CacheFile::build(&pb).await?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(chf.code_path)
            .await
            .into_diagnostic()?;
        for line in self.edit.code_block.lines() {
            file.write_all(line.as_bytes())
                .await
                .into_diagnostic()?;
            file.write_all(b"\n")
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
        if qs.qs_slug.is_none() {
            return Ok(());
        }

        self.edit.code_block = TextArea::default();

        let pb = get_question_index(&IdSlug::Slug(qs.qs_slug.clone().unwrap_or_default())).await?;
        let chf = CacheFile::build(&pb).await?;
        if !chf.code_path.exists() {
            glob_leetcode()
                .await
                .get_qs_detail(IdSlug::Slug(qs.qs_slug.clone().unwrap()), false)
                .await?;
        }

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
            self.edit.code_block.insert_str(line);
            self.edit.code_block.insert_newline();
        }
        self.edit.code_block.delete_newline();

        Ok(())
    }
}

// base
impl<'app_lf> App<'app_lf> {
    pub async fn new(events: EventsHandler) -> App<'app_lf> {
        let tab0 = select::SelectQS::new().await;
        let tab1 = EditCode::new();
        let tab2 = topic::TopicTagsQS::new().await;
        let tab3 = infos::KeyMaps::new();
        Self {
            titles: vec!["select", "edit", "select with topic", "infos"],
            tab_index: TuiIndex::Select,

            select: tab0,
            edit: tab1,
            topic: tab2,
            infos: tab3,

            cur_qs: Question::default(),

            pop_temp: false,
            temp_str: String::new(),

            save_code: false,
            next_key: next_key::NextKey { keymaps: Vec::new(), times: 0 },

            events,
            ..Default::default()
        }
    }
    pub fn next_tab(&mut self) -> bool {
        self.tab_index.next();
        true
    }
    pub fn prev_tab(&mut self) -> bool {
        self.tab_index.prev();
        true
    }
    pub fn goto_tab(&mut self, index: TuiIndex) -> bool {
        if matches!(index, TuiIndex::Edit) {
            match self.tab_index {
                TuiIndex::Select => self.get_qs_detail(IdSlug::Id(self.select.current_qs()), false),
                TuiIndex::Topic => {
                    let qs_slug = self.topic.cur_qs_slug();
                    if let Some(slug) = qs_slug {
                        self.get_qs_detail(IdSlug::Slug(slug), false);
                    }
                },
                _ => {},
            }
        }
        self.tab_index = index;
        true
    }
}
