pub mod cmds;
use std::path::PathBuf;

use lcode_config::global::G_USER_CONFIG;
use leetcode_api::leetcode::resps::{
    checkin::TotalPoints, pass_qs::PassData, user_data::UserStatus,
};
use ratatui::widgets::ListItem;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct Info<'tab3> {
    pub keymap: cmds::keymaps::KeymapState<'tab3>,

    pub user_status: UserStatus,

    pub points:      TotalPoints,
    pub pass_data:   PassData,
    pub avatar_path: PathBuf,
}

// keymaps
impl<'tab3> Info<'tab3> {
    pub fn new() -> Self {
        let mut pat = Vec::with_capacity(G_USER_CONFIG.keymap.map_set.len() + 1);
        pat.push(ListItem::new(
            "⭐ Give the project a star, cursor here Press Enter",
        ));

        let a = G_USER_CONFIG
            .keymap
            .map_set
            .iter()
            .map(|v| ListItem::new(v.to_string()));
        pat.extend(a);
        Self {
            keymap: cmds::keymaps::KeymapState::new(pat),
            // image_status:ThreadProtocol::new(tx, inner),
            ..Default::default()
        }
    }

    pub fn trigger(&self) -> bool {
        self.keymap.trigger()
    }

    pub fn first_item(&mut self) -> bool {
        self.keymap.first()
    }
    pub fn last_item(&mut self) -> bool {
        self.keymap.last()
    }
    pub fn prev_item(&mut self) -> bool {
        self.keymap.prev()
    }
    pub fn next_item(&mut self) -> bool {
        self.keymap.next()
    }
}
