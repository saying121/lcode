use ratatui::{
    style::{Color, Style},
    widgets::{ListItem, ListState},
};

pub struct KeyMaps<'tab3> {
    pub keymaps_state: ListState,
    pub keymaps_items: Vec<ListItem<'tab3>>,
}

// keymaps
impl<'tab3> KeyMaps<'tab3> {
    pub fn new() -> Self {
        Self {
            keymaps_items: vec![
                ListItem::new("Give the project a star, cursor here Press o or Enter")
                        .style(Style::default().fg(Color::LightCyan)),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Global keymap")
                        .style(Style::default().fg(Color::LightCyan)),
                ListItem::new(""),
                ListItem::new("Shift-Tab/Left     : Prev tab"),
                ListItem::new("Tab/Right          : Next tab"),
                ListItem::new("Ctrl-q             : Exit"),
                ListItem::new("Ctrl-l             : Refresh screen"),
                ListItem::new("gg/G               : Top/Bottom"),
                ListItem::new("j/k                : Up/Down"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Tab0/select")
                    .style(Style::default().fg(Color::LightCyan)),
                ListItem::new(""),
                ListItem::new("o                  : Open with your editor"),
                ListItem::new("C                  : Edit config"),
                ListItem::new("Enter              : Go to edit tab"),
                ListItem::new("S                  : Sync question information"),
                ListItem::new("Ctrl-r             : Re get current question"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Tab1/edit")
                    .style(Style::default().fg(Color::LightCyan)),
                ListItem::new(""),
                ListItem::new("Ctrl-p             : Toggle submit menu"),
                ListItem::new("S                  : Submit code(just show submit menu)"),
                ListItem::new("T                  : Test code(just show submit menu)"),
                ListItem::new("Ctrl-s             : Toggle Submit Result"),
                ListItem::new("Ctrl-t             : Toggle Test Result"),
                ListItem::new("Ctrl-r             : Re get current question, notice it will reget question by tab1 info"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Tab2/filter with topic")
                    .style(Style::default().fg(Color::LightCyan)),
                ListItem::new(""),
                ListItem::new("Ctrl-l             : Go to right"),
                ListItem::new("Ctrl-h             : Go to left"),
                ListItem::new("Ctrl-k             : Go to up"),
                ListItem::new("Ctrl-j             : Go to down"),
                ListItem::new("Enter(all topic)   : Add topic"),
                ListItem::new("Enter(user topic)  : Remove topic"),
                ListItem::new("Enter(questions)   : Confirm"),
                ListItem::new("S                  : Sync info"),
                ListItem::new("o                  : Open with your editor"),
                ListItem::new(""),
                ListItem::new("--------------------------------------------------------"),
                ListItem::new("Tab3/keymaps")
                    .style(Style::default().fg(Color::LightCyan)),
                ListItem::new(""),
                ListItem::new("gg/G               : Top/Bottom"),
            ],
            keymaps_state: ListState::default(),
        }
    }

    pub fn first_keymap(&mut self) {
        self.keymaps_state.select(Some(0));
    }
    pub fn last_keymap(&mut self) {
        self.keymaps_state
            .select(Some(self.keymaps_items.len() - 1));
    }
    pub fn prev_keymap(&mut self) {
        let i = match self.keymaps_state.selected() {
            Some(i) => (self.keymaps_items.len() + i - 1) % self.keymaps_items.len(),
            None => 0,
        };
        self.keymaps_state.select(Some(i));
    }
    pub fn next_keymap(&mut self) {
        let i = match self.keymaps_state.selected() {
            Some(i) => (i + 1) % self.keymaps_items.len(),
            None => 0,
        };
        self.keymaps_state.select(Some(i));
    }
}
