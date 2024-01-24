use std::mem;

use crossterm::event::KeyEvent;
use lcode_config::{config::global::G_USER_CONFIG, keymap::KeyMap};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct NextKey {
    pub(crate) keymaps: Vec<&'static KeyMap>,
    /// current tap times
    pub(crate) times:   usize,
}

impl NextKey {
    pub fn store_next(&mut self, keyevent: KeyEvent) {
        self.times = 1;
        self.keymaps = G_USER_CONFIG
            .keymap
            .keymap
            .iter()
            .filter(|v| v.keys.len() > 1 && v.keys[0] == keyevent.into())
            .collect();
    }
    pub fn have_next(&self) -> bool {
        !self.keymaps.is_empty()
    }
    pub fn handle_key(&mut self, keyevent: KeyEvent) -> Option<&'static String> {
        self.times += 1;

        self.keymaps = mem::take(&mut self.keymaps)
            .into_iter()
            .filter(|v| v.keys.len() >= self.times && v.keys[self.times - 1] == keyevent.into())
            .collect();
        match self
            .keymaps
            .iter()
            .position(|v| v.keys.len() == self.times)
        {
            Some(i) => {
                let res = &self.keymaps[i].action;
                self.clear();
                Some(res)
            },
            None => None,
        }
    }
    pub(crate) fn clear(&mut self) {
        self.times = 0;
        self.keymaps.clear();
    }
}
