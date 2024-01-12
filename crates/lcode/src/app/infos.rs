use lcode_config::config::global::USER_CONFIG;
use ratatui::widgets::{ListItem, ListState};

pub struct KeyMaps<'tab3> {
    pub keymaps_state: ListState,
    pub keymaps_items: Vec<ListItem<'tab3>>,
}

// keymaps
impl<'tab3> KeyMaps<'tab3> {
    pub fn new() -> Self {
        let mut pat = Vec::with_capacity(USER_CONFIG.keymap.keymap.len());
        pat.push(ListItem::new(
            "Give the project a star, cursor here Press o or Enter",
        ));

        let a: Vec<ListItem> = USER_CONFIG
            .keymap
            .keymap
            .iter()
            .map(|v| ListItem::new(v.to_string()))
            .collect();
        pat.extend(a);
        Self {
            keymaps_items: pat,
            keymaps_state: ListState::default(),
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
