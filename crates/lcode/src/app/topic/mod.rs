use crossterm::event::Event as CrossEvent;
use leetcode_api::{
    dao::query::{self, Query},
    entities::{new_index, topic_tags},
};
use ratatui::widgets::ListState;
use rayon::prelude::*;
use tui_textarea::{Input, TextArea};

use super::TuiMode;
use crate::fuzzy_search::filter;

/// tui layout position
///
/// |              |              |             |
/// | `AllTopics`  | `Difficulty` |             |
/// | ==========   | ==========   | `Questions` |
/// | `UserTopics` |              |             |
/// |              |              |             |
#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub enum Tab2Panel {
    #[default]
    AllTopics,
    UserTopics,
    Difficulty,
    Questions,
}

impl Tab2Panel {
    fn left(&mut self) {
        *self = match self {
            Self::AllTopics | Self::Difficulty => Self::AllTopics,
            Self::UserTopics => Self::UserTopics,
            Self::Questions => Self::Difficulty,
        }
    }
    fn right(&mut self) {
        *self = match self {
            Self::AllTopics | Self::UserTopics => Self::Difficulty,
            Self::Difficulty | Self::Questions => Self::Questions,
        }
    }
    fn up(&mut self) {
        *self = match self {
            Self::AllTopics | Self::UserTopics => Self::AllTopics,
            Self::Difficulty => Self::Difficulty,
            Self::Questions => Self::Questions,
        }
    }
    fn down(&mut self) {
        *self = match self {
            Self::AllTopics | Self::UserTopics => Self::UserTopics,
            Self::Difficulty => Self::Difficulty,
            Self::Questions => Self::Questions,
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct DiffState {
    pub user_diff:       String,
    pub difficultys:     Box<[String]>,
    pub diff_list_state: ListState,
}

impl DiffState {
    pub fn new(difficultys: Box<[String]>) -> Self {
        Self {
            user_diff: String::new(),
            difficultys,
            diff_list_state: ListState::default(),
        }
    }
    pub fn toggle_diff(&mut self) {
        let index = self
            .diff_list_state
            .selected()
            .unwrap_or_default();
        let diff = self
            .difficultys
            .get(index)
            .expect("get difficulty failed");
        if self.user_diff == *diff {
            self.user_diff = String::new();
        }
        else {
            self.user_diff.clone_from(diff);
        }
    }
    pub fn first(&mut self) {
        self.diff_list_state.select(Some(0));
    }
    pub fn last(&mut self) {
        self.diff_list_state
            .select(Some(self.difficultys.len()));
    }
    pub fn prev(&mut self) {
        let len = self.difficultys.len().max(1);
        let i = self
            .diff_list_state
            .selected()
            .map_or(0, |i| (len + i - 1) % len);
        self.diff_list_state.select(Some(i));
    }
    pub fn next(&mut self) {
        let len = self.difficultys.len().max(1);
        let i = self
            .diff_list_state
            .selected()
            .map_or(0, |i| (len + i + 1) % len);
        self.diff_list_state.select(Some(i));
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct TopicTagState {
    pub topic_tags:       Box<[topic_tags::Model]>,
    pub topic_tags_state: ListState,
}

impl TopicTagState {
    pub fn new(topic_tags: Box<[topic_tags::Model]>) -> Self {
        Self {
            topic_tags,
            topic_tags_state: ListState::default(),
        }
    }
    pub fn first_topic(&mut self) {
        self.topic_tags_state.select(Some(0));
    }
    pub fn last_topic(&mut self) {
        self.topic_tags_state
            .select(Some(self.topic_tags.len() - 1));
    }
    pub fn next_topic(&mut self) {
        let i = self
            .topic_tags_state
            .selected()
            .map_or(0, |i| {
                i.saturating_add(1)
                    .min(self.topic_tags.len().saturating_sub(1))
            });
        self.topic_tags_state.select(Some(i));
    }
    pub fn prev_topic(&mut self) {
        let i = self
            .topic_tags_state
            .selected()
            .map_or(0, |i| i.saturating_sub(1));
        self.topic_tags_state.select(Some(i));
    }
}

#[derive(Clone)]
#[derive(Default)]
#[derive(Debug)]
pub struct TopicTagsQS<'tab2> {
    pub topic_state: TopicTagState,

    pub all_topic_qs:            Box<[new_index::Model]>,
    pub filtered_topic_qs_state: ListState,
    pub filtered_qs:             Box<[new_index::Model]>,

    pub user_topic_tags:            Vec<String>,
    pub user_topic_tags_translated: Vec<String>,
    pub user_topic_tags_state:      ListState,

    pub sync_state: bool,
    pub cur_perc:   f64,

    pub index: Tab2Panel,

    pub text_line:       TextArea<'tab2>,
    pub input_line_mode: TuiMode,

    pub diff_state: DiffState,

    pub ac_status: Box<[(String, u32, u32)]>,
}

impl<'tab2> TopicTagsQS<'tab2> {
    pub fn keymap_insert(&mut self, event: CrossEvent) -> bool {
        match event.into() {
            Input { key: tui_textarea::Key::Esc, .. } => self.be_out_edit(),
            Input { key: tui_textarea::Key::Enter, .. } => false,
            input => self.text_line.input(input), // Use default key mappings in insert mode(emacs)
        };
        self.refresh_filter_by_input();
        true
    }
    pub fn be_out_edit(&mut self) -> bool {
        self.input_line_mode = TuiMode::OutEdit;
        true
    }
    pub fn enter_input_line(&mut self) -> bool {
        self.input_line_mode = TuiMode::Insert;
        true
    }

    pub fn up(&mut self) -> bool {
        match self.index {
            Tab2Panel::AllTopics => self.prev_topic(),
            Tab2Panel::UserTopics => self.prev_user_topic(),
            Tab2Panel::Difficulty => self.prev_diff(),
            Tab2Panel::Questions => self.prev_qs(),
        }
        true
    }
    pub fn down(&mut self) -> bool {
        match self.index {
            Tab2Panel::AllTopics => self.next_topic(),
            Tab2Panel::UserTopics => self.next_user_topic(),
            Tab2Panel::Difficulty => self.next_diff(),
            Tab2Panel::Questions => self.next_qs(),
        }
        true
    }
    pub fn panel_left(&mut self) -> bool {
        self.index.left();
        true
    }
    pub fn panel_right(&mut self) -> bool {
        self.index.right();
        true
    }
    pub fn panel_up(&mut self) -> bool {
        self.index.up();
        true
    }
    pub fn panel_down(&mut self) -> bool {
        self.index.down();
        true
    }
    pub fn top(&mut self) -> bool {
        match self.index {
            Tab2Panel::AllTopics => self.first_topic(),
            Tab2Panel::UserTopics => self.first_user_topic(),
            Tab2Panel::Difficulty => self.first_diff(),
            Tab2Panel::Questions => self.first_qs(),
        }
        true
    }
    pub fn bottom(&mut self) -> bool {
        match self.index {
            Tab2Panel::AllTopics => self.last_topic(),
            Tab2Panel::UserTopics => self.last_user_topic(),
            Tab2Panel::Difficulty => self.last_diff(),
            Tab2Panel::Questions => self.last_qs(),
        }
        true
    }
    pub async fn toggle_cursor(&mut self) -> bool {
        match self.index {
            Tab2Panel::AllTopics => self.add_user_topic().await,
            Tab2Panel::UserTopics => self.rm_user_topic().await,
            Tab2Panel::Difficulty => self.toggle_diff().await,
            Tab2Panel::Questions => {},
        }
        true
    }
}

// for `difficultys`
impl<'tab2> TopicTagsQS<'tab2> {
    pub async fn toggle_diff(&mut self) {
        self.diff_state.toggle_diff();

        self.refresh_filter_by_topic_diff()
            .await;
        self.refresh_filter_by_input();
    }
    pub fn prev_diff(&mut self) {
        self.diff_state.prev();
    }
    pub fn next_diff(&mut self) {
        self.diff_state.next();
    }
    pub fn first_diff(&mut self) {
        self.diff_state.first();
    }
    pub fn last_diff(&mut self) {
        self.diff_state.last();
    }
}

impl<'tab2> TopicTagsQS<'tab2> {
    pub async fn new() -> TopicTagsQS<'tab2> {
        let (new_index, topic_tags, ac_status) = Self::base_info().await;

        Self {
            topic_state: TopicTagState::new(topic_tags),

            all_topic_qs: new_index.clone(),
            filtered_topic_qs_state: ListState::default(),
            filtered_qs: new_index,

            user_topic_tags: vec![],
            user_topic_tags_translated: vec![],
            user_topic_tags_state: ListState::default(),

            sync_state: false,
            cur_perc: 0.0,

            index: Tab2Panel::AllTopics,

            text_line: TextArea::default(),
            input_line_mode: TuiMode::default(),

            diff_state: DiffState::new(
                ac_status
                    .iter()
                    .map(|v| v.0.clone())
                    .collect(),
            ),

            ac_status,
        }
    }

    /// return `new_index`, `topic_tags`, `ac_status`
    pub async fn base_info() -> (
        Box<[new_index::Model]>,
        Box<[topic_tags::Model]>,
        Box<[(String, u32, u32)]>,
    ) {
        let (all_qs_res, topic_res, status) = tokio::join!(
            query::Query::query_all_new_index(None),
            query::Query::query_all_topic(),
            query::Query::query_status()
        );
        (
            all_qs_res.unwrap_or_default().into(),
            topic_res.unwrap_or_default().into(),
            status.unwrap_or_default().into(),
        )
    }

    pub fn update_percent(&mut self, cur_perc: f64) {
        self.cur_perc = cur_perc;
    }
    /// refresh `filtered_qs`
    pub fn refresh_filter_by_input(&mut self) {
        self.filtered_qs = self
            .all_topic_qs
            .par_iter()
            .filter(|&v| filter(&self.text_line.lines()[0], &v.to_string()))
            .cloned()
            .collect();
    }
    /// refresh `all_topic_qs`
    pub async fn refresh_filter_by_topic_diff(&mut self) {
        if self.user_topic_tags.is_empty() {
            self.all_topic_qs = Query::query_all_new_index(Some(self.diff_state.user_diff.clone()))
                .await
                .unwrap_or_default()
                .into();
        }
        else {
            let diff = self.diff_state.user_diff.clone();
            self.all_topic_qs = Query::query_by_topic(&self.user_topic_tags, Some(diff))
                .await
                .unwrap_or_default()
                .into();
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
            self.user_topic_tags.remove(cur_top);
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
            .topic_state
            .topic_tags_state
            .selected()
            .unwrap_or_default();

        let (topic_slug, translated_slug) = self
            .topic_state
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
            self.user_topic_tags_translated.push(
                if translated_slug.is_empty() {
                    topic_slug.clone()
                }
                else {
                    translated_slug
                },
            );
            self.user_topic_tags.push(topic_slug);
        }
        self.refresh_filter_by_topic_diff()
            .await;
        self.refresh_filter_by_input();
    }

    // topic_tags //////////////////////////////////
    pub fn first_topic(&mut self) {
        self.topic_state.first_topic();
    }
    pub fn last_topic(&mut self) {
        self.topic_state.last_topic();
    }
    pub fn next_topic(&mut self) {
        self.topic_state.next_topic();
    }
    pub fn prev_topic(&mut self) {
        self.topic_state.prev_topic();
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
}

// user topic tags
impl<'tab2> TopicTagsQS<'tab2> {
    pub fn prev_user_topic(&mut self) {
        let index = self
            .user_topic_tags_state
            .selected()
            .map_or(0, |i| i.saturating_sub(1));
        self.user_topic_tags_state
            .select(Some(index));
    }

    pub fn next_user_topic(&mut self) {
        let index = self
            .user_topic_tags_state
            .selected()
            .map_or(0, |i| {
                i.saturating_add(1).min(
                    self.user_topic_tags
                        .len()
                        .saturating_sub(1),
                )
            });
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
