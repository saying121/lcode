use std::path::PathBuf;

use lcode_config::global::G_USER_CONFIG;
use leetcode_api::leetcode::resps::{
    checkin::TotalPoints, pass_qs::PassData, user_data::UserStatus,
};
use ratatui::widgets::{ListItem, ListState};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Infos<'tab3> {
    pub keymaps_state: ListState,
    pub keymaps_items: Vec<ListItem<'tab3>>,
    pub user_status:   UserStatus,

    pub points:      TotalPoints,
    pub pass_data:   PassData,
    pub avatar_path: PathBuf,
}

// keymaps
impl<'tab3> Infos<'tab3> {
    pub fn new() -> Self {
        let mut pat = Vec::with_capacity(G_USER_CONFIG.keymap.keymap.len() + 1);
        pat.push(ListItem::new(
            "⭐ Give the project a star, cursor here Press Enter",
        ));

        let a = G_USER_CONFIG
            .keymap
            .keymap
            .iter()
            .map(|v| ListItem::new(v.to_string()));
        pat.extend(a);
        Self {
            // image_status:ThreadProtocol::new(tx, inner),
            keymaps_items: pat,
            ..Default::default()
        }
    }

    pub fn trigger(&self) -> bool {
        let a = self
            .keymaps_state
            .selected()
            .unwrap_or_default();
        if a == 0 {
            crate::star();
        }
        false
    }

    pub fn first_item(&mut self) -> bool {
        self.keymaps_state.select(Some(0));
        true
    }
    pub fn last_item(&mut self) -> bool {
        self.keymaps_state
            .select(Some(self.keymaps_items.len() - 1));
        true
    }
    pub fn prev_item(&mut self) -> bool {
        let i = match self.keymaps_state.selected() {
            Some(i) => (self.keymaps_items.len() + i - 1) % self.keymaps_items.len(),
            None => 0,
        };
        self.keymaps_state.select(Some(i));
        true
    }
    pub fn next_item(&mut self) -> bool {
        let i = match self.keymaps_state.selected() {
            Some(i) => (i + 1) % self.keymaps_items.len(),
            None => 0,
        };
        self.keymaps_state.select(Some(i));
        true
    }
}
