use ratatui::widgets::{ListItem, ListState};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct KeymapState<'key> {
    pub keymaps_state: ListState,
    pub keymaps_items: Vec<ListItem<'key>>,
}

impl<'key> KeymapState<'key> {
    pub fn new(keymaps_items: Vec<ListItem<'key>>) -> Self {
        Self {
            keymaps_state: ListState::default(),
            keymaps_items,
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

    pub fn first(&mut self) -> bool {
        self.keymaps_state.select(Some(0));
        true
    }
    pub fn last(&mut self) -> bool {
        self.keymaps_state
            .select(Some(self.keymaps_items.len() - 1));
        true
    }
    pub fn prev(&mut self) -> bool {
        let i = match self.keymaps_state.selected() {
            Some(i) => (self.keymaps_items.len() + i - 1) % self.keymaps_items.len(),
            None => 0,
        };
        self.keymaps_state.select(Some(i));
        true
    }
    pub fn next(&mut self) -> bool {
        let i = match self.keymaps_state.selected() {
            Some(i) => (i + 1) % self.keymaps_items.len(),
            None => 0,
        };
        self.keymaps_state.select(Some(i));
        true
    }
}
