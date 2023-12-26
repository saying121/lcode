use crossterm::event::Event as CrossEvent;
use miette::Result;
use ratatui::{
    style::{Style, Styled},
    widgets::{Block, Borders, ListState},
};
use rayon::prelude::*;
use tui_textarea::{Input, TextArea};

use super::{Tab2Panel, TuiMode};
use crate::{
    dao::query_topic_tags,
    editor::{edit, CodeTestFile},
    entities::{new_index, topic_tags},
    fuzzy_search::filter,
    leetcode::IdSlug,
};

pub struct TopicTagsQS<'tab2> {
    pub topic_tags:       Vec<topic_tags::Model>,
    pub topic_tags_state: ListState,

    pub all_topic_qs:            Vec<new_index::Model>,
    pub filtered_topic_qs_state: ListState,
    pub filtered_qs:             Vec<new_index::Model>,

    pub user_topic_tags:            Vec<String>,
    pub user_topic_tags_translated: Vec<String>,
    pub user_topic_tags_state:      ListState,

    pub sync_state: bool,
    pub cur_perc:   f64,

    pub index: Tab2Panel,

    pub text_line:       TextArea<'tab2>,
    pub input_line_mode: TuiMode,

    pub user_diff:         String,
    pub difficultys:       Vec<String>,
    pub difficultys_state: ListState,

    pub ac_status: Vec<(String, u32, u32)>,
}

impl<'tab2> TopicTagsQS<'tab2> {
    pub fn keymap_insert(&mut self, event: CrossEvent) {
        match event.into() {
            Input {
                key: tui_textarea::Key::Esc,
                ..
            } => self.be_out_edit(),
            input => {
                self.text_line.input(input); // Use default key mappings in insert mode(emacs)
            },
        }
        self.refresh_filter_by_input();
    }
    pub fn be_out_edit(&mut self) {
        self.input_line_mode = TuiMode::OutEdit;
    }
    pub fn enter_input_line(&mut self) {
        self.input_line_mode = TuiMode::Insert;
    }

    pub fn up(&mut self) {
        match self.index {
            Tab2Panel::AllTopics => self.prev_topic(),
            Tab2Panel::UserTopics => self.prev_user_topic(),
            Tab2Panel::Difficulty => self.prev_diff(),
            Tab2Panel::Questions => self.prev_qs(),
        }
    }
    pub fn down(&mut self) {
        match self.index {
            Tab2Panel::AllTopics => self.next_topic(),
            Tab2Panel::UserTopics => self.next_user_topic(),
            Tab2Panel::Difficulty => self.next_diff(),
            Tab2Panel::Questions => self.next_qs(),
        }
    }
    pub fn panel_left(&mut self) {
        self.index.left();
    }
    pub fn panel_right(&mut self) {
        self.index.right();
    }
    pub fn panel_up(&mut self) {
        self.index.up();
    }
    pub fn panel_down(&mut self) {
        self.index.down();
    }
    pub fn top(&mut self) {
        match self.index {
            Tab2Panel::AllTopics => self.first_topic(),
            Tab2Panel::UserTopics => self.first_user_topic(),
            Tab2Panel::Difficulty => self.first_diff(),
            Tab2Panel::Questions => self.first_qs(),
        }
    }
    pub fn bottom(&mut self) {
        match self.index {
            Tab2Panel::AllTopics => self.last_topic(),
            Tab2Panel::UserTopics => self.last_user_topic(),
            Tab2Panel::Difficulty => self.last_diff(),
            Tab2Panel::Questions => self.last_qs(),
        }
    }
    pub async fn toggle_cursor(&mut self) {
        match self.index {
            Tab2Panel::AllTopics => self.add_user_topic().await,
            Tab2Panel::UserTopics => self.rm_user_topic().await,
            Tab2Panel::Difficulty => self.toggle_diff().await,
            Tab2Panel::Questions => {},
        }
    }
}

// for `difficultys`
impl<'tab2> TopicTagsQS<'tab2> {
    pub async fn toggle_diff(&mut self) {
        let index = self
            .difficultys_state
            .selected()
            .unwrap_or_default();
        let diff = self
            .difficultys
            .get(index)
            .unwrap();
        if self.user_diff == *diff {
            self.user_diff = String::new();
        }
        else {
            self.user_diff = diff.clone();
        }
        self.refresh_filter_by_topic_diff()
            .await;
        self.refresh_filter_by_input();
    }
    pub fn prev_diff(&mut self) {
        let len = self.difficultys.len().max(1);
        let i = self
            .difficultys_state
            .selected()
            .map_or(0, |i| (len + i - 1) % len);
        self.difficultys_state
            .select(Some(i));
    }
    pub fn next_diff(&mut self) {
        let len = self.difficultys.len().max(1);
        let i = self
            .difficultys_state
            .selected()
            .map_or(0, |i| (len + i + 1) % len);
        self.difficultys_state
            .select(Some(i));
    }
    pub fn first_diff(&mut self) {
        self.difficultys_state
            .select(Some(0));
    }
    pub fn last_diff(&mut self) {
        self.difficultys_state
            .select(Some(self.difficultys.len()));
    }
}

impl<'tab2> TopicTagsQS<'tab2> {
    pub async fn new() -> TopicTagsQS<'tab2> {
        let base = Self::base_info().await;
        let all_qs = base.0;
        let status = base.2;
        let mut text_line = TextArea::default();
        text_line.set_block(
            Block::default()
                .borders(Borders::ALL)
                .set_style(Style::default())
                .title("Press `e` for input"),
        );

        Self {
            topic_tags: base.1,
            topic_tags_state: ListState::default(),

            all_topic_qs: all_qs.clone(),
            filtered_topic_qs_state: ListState::default(),
            filtered_qs: all_qs,

            user_topic_tags: vec![],
            user_topic_tags_translated: vec![],
            user_topic_tags_state: ListState::default(),

            sync_state: false,
            cur_perc: 0.0,

            index: Tab2Panel::AllTopics,

            text_line,
            input_line_mode: TuiMode::default(),

            user_diff: String::new(),
            difficultys: status
                .iter()
                .map(|v| v.0.clone())
                .collect(),
            difficultys_state: ListState::default(),

            ac_status: status,
        }
    }

    /// return `new_index`, `topic_tags`, `ac_status`
    async fn base_info() -> (
        Vec<new_index::Model>,
        Vec<topic_tags::Model>,
        Vec<(String, u32, u32)>,
    ) {
        let (all_qs_res, topic_res, status) = tokio::join!(
            query_topic_tags::query_all_new_index(None),
            query_topic_tags::query_all_topic(),
            query_topic_tags::query_status()
        );
        (
            all_qs_res.unwrap_or_default(),
            topic_res.unwrap_or_default(),
            status.unwrap_or_default(),
        )
    }

    /// refresh `all_topic_qs`, `filtered_qs`, `topic_tags`, `difficultys`
    pub async fn sync_new_done(&mut self) {
        self.sync_state = false;
        let base = Self::base_info().await;
        self.all_topic_qs = base.0;
        self.topic_tags = base.1;
        self.difficultys = base
            .2
            .iter()
            .map(|v| v.0.clone())
            .collect();
        self.ac_status = base.2;

        self.refresh_filter_by_topic_diff()
            .await;
        self.refresh_filter_by_input();
    }
    pub fn update_percent(&mut self, cur_perc: f64) {
        self.cur_perc = cur_perc;
    }
    /// refresh `filtered_qs`
    pub fn refresh_filter_by_input(&mut self) {
        self.filtered_qs = self
            .all_topic_qs
            .clone()
            .into_par_iter()
            .filter(|v| filter(&self.text_line.lines()[0], &"", &v.to_string(), 1))
            .collect::<Vec<new_index::Model>>();
    }
    /// refresh `all_topic_qs`
    async fn refresh_filter_by_topic_diff(&mut self) {
        if self.user_topic_tags.is_empty() {
            self.all_topic_qs = query_topic_tags::query_all_new_index(Some(self.user_diff.clone()))
                .await
                .unwrap_or_default();
        }
        else {
            let diff = self.user_diff.clone();
            self.all_topic_qs = query_topic_tags::query_by_topic(&self.user_topic_tags, Some(diff))
                .await
                .unwrap_or_default();
        }
    }
}

// all topic tags, add remove topic
impl<'tab2> TopicTagsQS<'tab2> {
    pub async fn rm_user_topic(&mut self) {
        let cur_top = self
            .user_topic_tags_state
            .selected()
            .unwrap_or_default();

        if !self.user_topic_tags.is_empty() {
            self.user_topic_tags
                .remove(cur_top);
            self.user_topic_tags_translated
                .remove(cur_top);
        }
        if cur_top >= self.user_topic_tags.len() {
            self.prev_user_topic();
        }

        self.refresh_filter_by_topic_diff()
            .await;
        self.refresh_filter_by_input();
    }

    pub async fn add_user_topic(&mut self) {
        let cur_top = self
            .topic_tags_state
            .selected()
            .unwrap_or_default();

        let (topic_slug, translated_slug) = self
            .topic_tags
            .get(cur_top)
            .map(|v| {
                (
                    v.topic_slug.clone(),
                    v.name_translated
                        .clone()
                        .unwrap_or_default(),
                )
            })
            .unwrap_or_default();

        if !self
            .user_topic_tags
            .contains(&topic_slug)
        {
            self.user_topic_tags_translated
                .push(
                    if translated_slug.is_empty() {
                        topic_slug.clone()
                    }
                    else {
                        translated_slug
                    },
                );
            self.user_topic_tags
                .push(topic_slug);
        }
        self.refresh_filter_by_topic_diff()
            .await;
        self.refresh_filter_by_input();
    }

    // topic_tags //////////////////////////////////
    pub fn first_topic(&mut self) {
        self.topic_tags_state
            .select(Some(0));
    }
    pub fn last_topic(&mut self) {
        self.topic_tags_state
            .select(Some(self.topic_tags.len() - 1));
    }
    pub fn next_topic(&mut self) {
        let i = self
            .topic_tags_state
            .selected()
            .map_or(0, |i| (i + 1) % self.topic_tags.len().max(1));
        self.topic_tags_state
            .select(Some(i));
    }
    pub fn prev_topic(&mut self) {
        let len = self.topic_tags.len().max(1);
        let i = self
            .topic_tags_state
            .selected()
            .map_or(0, |i| (len + i - 1) % len);
        self.topic_tags_state
            .select(Some(i));
    }
}

// filtered questions
impl<'tab2> TopicTagsQS<'tab2> {
    pub fn next_qs(&mut self) {
        let index = self
            .filtered_topic_qs_state
            .selected()
            .map_or(0, |i| (i + 1) % self.filtered_qs.len().max(1));
        self.filtered_topic_qs_state
            .select(Some(index));
    }
    pub fn prev_qs(&mut self) {
        let len = self.filtered_qs.len().max(1);
        let index = self
            .filtered_topic_qs_state
            .selected()
            .map_or(0, |i| (len + i - 1) % len);
        self.filtered_topic_qs_state
            .select(Some(index));
    }
    pub fn first_qs(&mut self) {
        self.filtered_topic_qs_state
            .select(Some(0));
    }
    pub fn last_qs(&mut self) {
        self.filtered_topic_qs_state
            .select(Some(self.filtered_qs.len() - 1));
    }
    pub fn cur_qs_slug(&self) -> Option<String> {
        let index = self
            .filtered_topic_qs_state
            .selected()
            .unwrap_or_default();
        self.filtered_qs
            .get(index)
            .map(|v| v.title_slug.clone())
    }
    /// edit cursor qs with outer editor
    pub async fn edit_cur_qs(&mut self) -> Result<()> {
        let qs_slug = self.cur_qs_slug();
        match qs_slug {
            Some(slug) => edit(IdSlug::Slug(slug), CodeTestFile::Code).await,
            None => Ok(()),
        }
    }
}

// user topic tags
impl<'tab2> TopicTagsQS<'tab2> {
    pub fn prev_user_topic(&mut self) {
        let len = self.user_topic_tags.len().max(1);
        let index = self
            .user_topic_tags_state
            .selected()
            .map_or(0, |i| (len + i - 1) % len);
        self.user_topic_tags_state
            .select(Some(index));
    }

    pub fn next_user_topic(&mut self) {
        let index = self
            .user_topic_tags_state
            .selected()
            .map_or(0, |i| (i + 1) % self.user_topic_tags.len().max(1));
        self.user_topic_tags_state
            .select(Some(index));
    }
    pub fn last_user_topic(&mut self) {
        self.user_topic_tags_state
            .select(Some(self.user_topic_tags.len() - 1));
    }
    pub fn first_user_topic(&mut self) {
        self.user_topic_tags_state
            .select(Some(0));
    }
}
