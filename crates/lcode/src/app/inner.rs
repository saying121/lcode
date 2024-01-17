use std::{sync::atomic::Ordering, time::Duration};

use leetcode_api::{
    dao::{get_question_index, query_all_index, save_info::CacheFile},
    leetcode::{
        question::qs_detail::Question,
        resps::{
            checkin::TotalPoints, run_res::RunResult, user_data::UserStatus, SubmitInfo, TestInfo,
        },
        IdSlug, CUR_QS_INDEX_NUM, CUR_TOPIC_QS_INDEX_NUM, TOTAL_QS_INDEX_NUM,
        TOTAL_TOPIC_QS_INDEX_NUM,
    },
};
use miette::{IntoDiagnostic, Result};
use notify_rust::Notification;
use tokio::{
    self,
    fs::{File, OpenOptions},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    join,
};
use tracing::error;
use tui_textarea::TextArea;

use super::{
    dispatch::next_key,
    edit::EditCode,
    infos, select,
    topic::{self, TopicTagsQS},
    TuiIndex,
};
use crate::{
    editor::{self, CodeTestFile},
    glob_leetcode,
    mytui::{
        myevent::{EventsHandler, UserEvent},
        term::Term,
    },
};
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

impl<'app> App<'app> {
    pub fn user_info_and_checkin(&self) {
        let tx = self.events.tx.clone();

        tokio::spawn(async move {
            let (u_st, points) = join!(
                glob_leetcode()
                    .await
                    .get_user_info(),
                glob_leetcode().await.get_points()
            );

            if let Ok(status) = &u_st {
                let avatar_path = glob_leetcode()
                    .await
                    .dow_user_avator(status)
                    .await;
                let body = format!("{}, checkin leetcode", status.username);

                if !status.checked_in_today {
                    let res = glob_leetcode()
                        .await
                        .daily_checkin()
                        .await;
                    if res.is_ok() {
                        tokio::task::spawn_blocking(move || {
                            Notification::new()
                                .appname("lcode")
                                .summary("Leetcode Checkin")
                                .body(&body)
                                .icon(
                                    avatar_path
                                        .as_os_str()
                                        .to_str()
                                        .unwrap_or_default(),
                                )
                                .show()
                                .ok();
                        });
                    }
                }
            }
            tx.send(UserEvent::UserInfo((
                u_st.unwrap_or_default(),
                points.unwrap_or_default(),
            )))
        });
    }

    pub fn get_status_done(&mut self, info: (UserStatus, TotalPoints)) {
        (self.user_status, self.points) = info;
    }
}

impl<'app_lf> App<'app_lf> {
    /// edit cursor qs with outer editor
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
    /// edit cursor qs with outer editor
    pub async fn topic_edit_cur_qs(&mut self) -> Result<()> {
        let qs_slug = self.topic.cur_qs_slug();
        if let Some(slug) = qs_slug {
            self.pause();
            editor::open(IdSlug::Slug(slug), CodeTestFile::Code).await?;
            self.r#continue();
        }
        Ok(())
    }
    /// send info for render tui
    pub fn render(&mut self) {
        self.events.render();
    }
    pub fn exit(&mut self) -> bool {
        self.events.exit();
        false
    }
    /// leave alter screen, and stop eventstream
    pub fn pause(&mut self) {
        Term::stop().ok();
        self.events.stop_events().ok();
    }
    /// enter alter screen, and start eventstream
    pub fn r#continue(&mut self) {
        Term::start().ok();
        self.events = EventsHandler::new();
        self.events.redraw_tui();
    }
}

impl<'app_lf> App<'app_lf> {
    pub fn sync_index(&mut self) -> bool {
        if self.select.sync_state {
            return false;
        }
        self.select.sync_state = true;
        let eve_tx = self.events.tx.clone();

        let handle = tokio::spawn(async move {
            if let Err(err) = glob_leetcode()
                .await
                .sync_problem_index()
                .await
            {
                error!("{}", err);
            }

            eve_tx
                .send(UserEvent::SyncDone)
                .unwrap();
        });
        let tx = self.events.tx.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(200)).await;
                let a = CUR_QS_INDEX_NUM.load(Ordering::Relaxed) as f64
                    / TOTAL_QS_INDEX_NUM.load(Ordering::Relaxed) as f64;
                if a <= 1.0 {
                    tx.send(UserEvent::Syncing(a)).ok();
                    tx.send(UserEvent::Render).ok();
                }
                if handle.is_finished() {
                    break;
                }
            }
        });
        true
    }
    /// refresh `all_questions`, `filtered_qs`
    pub async fn sync_done(&mut self) {
        self.select.sync_state = false;
        let questions = query_all_index()
            .await
            .unwrap_or_default();
        self.select.all_questions = questions;
        self.select.filter_by_input();

        self.render();
    }
    pub fn sync_new(&mut self) -> bool {
        if self.topic.sync_state {
            return false;
        }

        self.topic.sync_state = true;
        let eve_tx = self.events.tx.clone();
        let handle = tokio::spawn(async move {
            if let Err(err) = glob_leetcode()
                .await
                .new_sync_index()
                .await
            {
                error!("{}", err);
            }

            eve_tx
                .send(UserEvent::SyncDoneNew)
                .unwrap();
        });
        let tx = self.events.tx.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(200)).await;
                let a = CUR_TOPIC_QS_INDEX_NUM.load(Ordering::Relaxed) as f64
                    / TOTAL_TOPIC_QS_INDEX_NUM.load(Ordering::Relaxed) as f64;
                if a <= 1.0 {
                    tx.send(UserEvent::SyncingNew(a))
                        .ok();
                    tx.send(UserEvent::Render).ok();
                }
                if handle.is_finished() {
                    break;
                }
            }
        });
        false
    }
    /// refresh `all_topic_qs`, `filtered_qs`, `topic_tags`, `difficultys`
    pub async fn sync_new_done(&mut self) {
        self.topic.sync_state = false;
        let base = TopicTagsQS::base_info().await;
        self.topic.all_topic_qs = base.0;
        self.topic.topic_tags = base.1;
        self.topic.difficultys = base
            .2
            .iter()
            .map(|v| v.0.clone())
            .collect();
        self.topic.ac_status = base.2;

        self.topic
            .refresh_filter_by_topic_diff()
            .await;
        self.topic
            .refresh_filter_by_input();

        self.render();
    }
    pub fn get_qs_detail(&self, idslug: IdSlug, force: bool) {
        let eve_tx = self.events.tx.clone();
        tokio::spawn(async move {
            let qs = glob_leetcode()
                .await
                .get_qs_detail(idslug, force)
                .await
                .unwrap_or_default();
            eve_tx
                .send(UserEvent::GetQsDone(Box::new(qs)))
                .unwrap();
        });
    }
    pub fn submit_code(&mut self) -> bool {
        let id: u32 = self
            .cur_qs
            .question_id
            .parse()
            .unwrap_or_default();

        // avoid repeated requests
        if self.edit.submitting {
            return false;
        }

        self.edit.submitting = true;
        let eve_tx = self.events.tx.clone();
        tokio::spawn(async move {
            // min id is 1
            let temp = if id > 0 {
                glob_leetcode()
                    .await
                    .submit_code(IdSlug::Id(id))
                    .await
                    .unwrap_or_default()
            }
            else {
                (SubmitInfo::default(), RunResult::default())
            };
            eve_tx
                .send(UserEvent::SubmitDone(Box::new(temp.1)))
                .unwrap();
        });
        false
    }

    pub fn test_code(&mut self) -> bool {
        let id = self
            .cur_qs
            .question_id
            .parse()
            .unwrap_or_default();

        // avoid repeated requests
        if self.edit.submitting {
            return false;
        }
        self.edit.submitting = true;

        let eve_tx = self.events.tx.clone();
        tokio::spawn(async move {
            // min id is 1
            let temp = if id > 0 {
                glob_leetcode()
                    .await
                    .test_code(IdSlug::Id(id))
                    .await
                    .unwrap_or_default()
            }
            else {
                (TestInfo::default(), RunResult::default())
            };
            eve_tx
                .send(UserEvent::TestDone(Box::new(temp.1)))
                .unwrap();
        });
        false
    }
    pub fn submit_done(&mut self, res: RunResult) {
        self.edit.submit_res = res;
        self.edit.show_submit_res = true;
        self.edit.submitting = false;
        self.render();
    }
    pub fn test_done(&mut self, res: RunResult) {
        self.edit.test_res = res;
        self.edit.show_test_res = true;
        self.edit.submitting = false;
        self.render();
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

        let pb = get_question_index(&IdSlug::Slug(
            qs.qs_slug
                .clone()
                .unwrap_or_default(),
        ))
        .await?;
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
            self.edit
                .code_block
                .insert_str(line);
            self.edit
                .code_block
                .insert_newline();
        }
        self.edit
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
        self.render();
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
