use ratatui::widgets::{ListItem, ListState};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct KeymapState<'key> {
    pub items: Vec<ListItem<'key>>,
    pub list_state: ListState,
}

impl<'key> KeymapState<'key> {
    pub fn new(keymaps_items: Vec<ListItem<'key>>) -> Self {
        Self {
            list_state: ListState::default(),
            items: keymaps_items,
        }
    }
    pub fn trigger(&self) -> bool {
        let a = self
            .list_state
            .selected()
            .unwrap_or_default();
        if a == 0 {
            crate::star();
        }
        false
    }

    pub fn first(&mut self) -> bool {
        self.list_state.select(Some(0));
        true
    }
    pub fn last(&mut self) -> bool {
        self.list_state
            .select(Some(self.items.len() - 1));
        true
    }
    pub fn prev(&mut self) -> bool {
        let i = match self.list_state.selected() {
            Some(i) => (self.items.len() + i - 1) % self.items.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
        true
    }
    pub fn next(&mut self) -> bool {
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
        true
    }
}
