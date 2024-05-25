pub mod cmds;

use crossterm::event::Event as CrossEvent;
use leetcode_api::{
    dao::query::{self, Query},
    entities::{new_index, topic_tags},
};
use rayon::prelude::*;
use tui_textarea::Input;

use self::cmds::{diff, intputline, question, sync_bar, topics};
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
#[derive(Default)]
#[derive(Debug)]
pub struct TopicTagsQS<'tab2> {
    pub topic:          topics::TopicsState,
    pub question_state: question::TopicQuestionState,
    pub sync_bar:       sync_bar::BarState,
    pub index:          Tab2Panel,
    pub inputline:      intputline::InputLine<'tab2>,
    pub difficulty:     diff::DiffState,
    pub ac_status:      Box<[(String, u32, u32)]>,
}

impl<'tab2> TopicTagsQS<'tab2> {
    pub fn keymap_insert(&mut self, event: CrossEvent) -> bool {
        match event.into() {
            Input { key: tui_textarea::Key::Esc, .. } => self.be_out_edit(),
            Input { key: tui_textarea::Key::Enter, .. } => false, // just one line so do nothing
            input => {
                // Use default key mappings in insert mode(emacs)
                let trigger = self.inputline.handle_input(input);
                if trigger {
                    self.refresh_filter_by_input();
                }
                trigger
            },
        }
    }
    pub fn be_out_edit(&mut self) -> bool {
        self.inputline.out_edit();
        true
    }
    pub fn enter_input_line(&mut self) -> bool {
        self.inputline.enter_input();
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
            Tab2Panel::Questions => true,
        }
    }
}

// for `difficulties`
impl<'tab2> TopicTagsQS<'tab2> {
    pub async fn toggle_diff(&mut self) -> bool {
        // the operate must trigger refresh
        self.difficulty.toggle_diff();

        self.refresh_filter_by_topic_diff()
            .await;
        self.refresh_filter_by_input();
        true
    }
    pub fn prev_diff(&mut self) {
        self.difficulty.prev();
    }
    pub fn next_diff(&mut self) {
        self.difficulty.next();
    }
    pub fn first_diff(&mut self) {
        self.difficulty.first();
    }
    pub fn last_diff(&mut self) {
        self.difficulty.last();
    }
}

impl<'tab2> TopicTagsQS<'tab2> {
    pub async fn new() -> TopicTagsQS<'tab2> {
        let (new_index, topic_tags, ac_status) = Self::base_info().await;

        Self {
            topic: topics::TopicsState::new(topic_tags),

            question_state: question::TopicQuestionState::new(new_index),

            sync_bar: sync_bar::BarState::default(),

            index: Tab2Panel::AllTopics,

            inputline: intputline::InputLine::default(),

            difficulty: diff::DiffState::new(
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

    pub fn update_percent(&mut self, perc: f64) {
        self.sync_bar.update(perc);
    }
    /// refresh `filtered_qs`
    pub fn refresh_filter_by_input(&mut self) {
        self.question_state.filtered_qs = self
            .question_state
            .all_qs
            .par_iter()
            .filter(|&v| filter(self.inputline.first_line(), &v.to_string()))
            .cloned()
            .collect();
    }
    /// refresh `all_qs`
    pub async fn refresh_filter_by_topic_diff(&mut self) {
        if self.topic.user_topic_tags.is_empty() {
            self.question_state.all_qs =
                Query::query_all_new_index(Some(self.difficulty.user_diff.clone()))
                    .await
                    .unwrap_or_default()
                    .into();
        }
        else {
            let diff = self.difficulty.user_diff.clone();
            self.question_state.all_qs =
                Query::query_by_topic(&self.topic.user_topic_tags, Some(diff))
                    .await
                    .unwrap_or_default()
                    .into();
        }
    }
}

// all topic tags, add remove topic
impl<'tab2> TopicTagsQS<'tab2> {
    /// remove a topic and refresh question
    pub async fn rm_user_topic(&mut self) -> bool {
        let trigger = self.topic.rm_user_topic();
        if trigger {
            self.refresh_filter_by_topic_diff()
                .await;
            self.refresh_filter_by_input();
        }
        trigger
    }

    /// return need refresh or not
    pub async fn add_user_topic(&mut self) -> bool {
        let trigger = self.topic.add_user_topic();
        if trigger {
            self.refresh_filter_by_topic_diff()
                .await;
            self.refresh_filter_by_input();
        }
        trigger
    }

    // topic_tags //////////////////////////////////
    pub fn first_topic(&mut self) {
        self.topic.first();
    }
    pub fn last_topic(&mut self) {
        self.topic.last();
    }
    pub fn next_topic(&mut self) {
        self.topic.next();
    }
    pub fn prev_topic(&mut self) {
        self.topic.prev();
    }
}

// filtered questions
impl<'tab2> TopicTagsQS<'tab2> {
    pub fn next_qs(&mut self) {
        self.question_state.next();
    }
    pub fn prev_qs(&mut self) {
        self.question_state.prev();
    }
    pub fn first_qs(&mut self) {
        self.question_state
            .filtered_topic_qs_state
            .select(Some(0));
    }
    pub fn last_qs(&mut self) {
        self.question_state.last();
    }
    pub fn cur_qs_slug(&self) -> Option<String> {
        self.question_state.cur_qs_slug()
    }
}

// user topic tags
impl<'tab2> TopicTagsQS<'tab2> {
    pub fn prev_user_topic(&mut self) {
        self.topic.prev_user();
    }

    pub fn next_user_topic(&mut self) {
        self.topic.next_user();
    }
    pub fn last_user_topic(&mut self) {
        self.topic.last_user();
    }
    pub fn first_user_topic(&mut self) {
        self.topic.first_user();
    }
}
