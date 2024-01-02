use std::{collections::HashSet, hash::Hash};

use crossterm::event::KeyCode;
use key_parse::{self, keymap::*};
use serde::{Deserialize, Serialize};

pub const PANEL_UP: &str = "panel_up";
pub const PANEL_DOWN: &str = "panel_down";
pub const PANEL_RIGHT: &str = "panel_right";
pub const PANEL_LEFT: &str = "panel_left";

pub const UP: &str = "up";
pub const DOWN: &str = "down";
pub const RIGHT: &str = "right";
pub const LEFT: &str = "left";

pub const TOGGLE_CURSOR: &str = "toggle";

pub const TOP: &str = "top";
pub const BOTTOM: &str = "bottom";

pub const REDRAW: &str = "redraw";
pub const EXIT: &str = "exit";

pub const EDIT_CODE_EDITOR: &str = "edit_code";
pub const EDIT_IN_TUI: &str = "edit_code_tui";

pub const TOGGLE_SUBMIT_RES: &str = "toggle_submit_res";
pub const TOGGLE_TEST_RES: &str = "toggle_test_res";
pub const TOGGLE_MENU: &str = "toggle_menu";

pub const TEST_CODE: &str = "test_code";
pub const SUBMIT_CODE: &str = "submit_code";
pub const RE_QS_DETAIL: &str = "re_get_qs";
pub const NEXT_TAB: &str = "next_tab";
pub const PREV_TAB: &str = "prev_tab";
pub const HEAD: &str = "head";
pub const TAIL: &str = "tail";
pub const SYNC_INDEX: &str = "sync_index";

pub const ESCAPE: &str = "escape";

#[derive(Clone, Serialize, Deserialize, Eq, Debug)]
pub struct KeyMap {
    pub keys:   Keys,
    pub action: String,
    #[serde(default)]
    pub desc:   String,
}

impl PartialEq for KeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action
    }
}

impl Hash for KeyMap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.action.hash(state);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct TuiKeyMap {
    pub keymap: HashSet<KeyMap>,
}

impl TuiKeyMap {
    pub fn add_keymap(&mut self, add: HashSet<KeyMap>) {
        for i in add {
            self.keymap.replace(i);
        }
    }
}

impl Default for TuiKeyMap {
    fn default() -> Self {
        // if have float panel that first care
        let keymap = HashSet::from([
            KeyMap {
                keys:   Keys(vec![Key::new(NO_CONTROL, KeyCode::Tab)]),
                action: NEXT_TAB.to_owned(),
                desc:   "next tab".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(SHIFT, KeyCode::BackTab)]),
                action: PREV_TAB.to_owned(),
                desc:   "prev tab".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(CTRL, KeyCode::Char('l'))]),
                action: REDRAW.to_owned(),
                desc:   "redraw ui".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(CTRL, KeyCode::Char('q'))]),
                action: EXIT.to_owned(),
                desc:   "exit lcode".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![
                    Key::new(NO_CONTROL, KeyCode::Char('g')),
                    Key::new(NO_CONTROL, KeyCode::Char('g')),
                ]),
                action: TOP.to_owned(),
                desc:   "go to top".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(SHIFT, KeyCode::Char('G'))]),
                action: BOTTOM.to_owned(),
                desc:   "go to bottom".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('o'))]),
                action: EDIT_CODE_EDITOR.to_owned(),
                desc:   "edit cursor question(or current question) with your editor".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('e'))]),
                action: EDIT_IN_TUI.to_owned(),
                desc:   "Enter input line or code block".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('l'))]),
                action: RIGHT.to_owned(),
                desc:   "panel content move right".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('h'))]),
                action: LEFT.to_owned(),
                desc:   "panel content move left".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('k'))]),
                action: UP.to_owned(),
                desc:   "panel content move up".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('j'))]),
                action: DOWN.to_owned(),
                desc:   "panel content move down".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(ALT, KeyCode::Char('h'))]),
                action: PANEL_LEFT.to_owned(),
                desc:   "switch to left panel(topic tags)".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(ALT, KeyCode::Char('l'))]),
                action: PANEL_RIGHT.to_owned(),
                desc:   "switch to right panel(topic tags)".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(ALT, KeyCode::Char('j'))]),
                action: PANEL_DOWN.to_owned(),
                desc:   "switch to down panel(topic tags)".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(ALT, KeyCode::Char('k'))]),
                action: PANEL_UP.to_owned(),
                desc:   "switch to up panel(topic tags)".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(SHIFT, KeyCode::Char('H'))]),
                action: HEAD.to_owned(),
                desc:   "To the first column of the panel content.".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(SHIFT, KeyCode::Char('L'))]),
                action: TAIL.to_owned(),
                desc:   "To the end column of the panel content.".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(NO_CONTROL, KeyCode::Esc)]),
                action: ESCAPE.to_owned(),
                desc:   "close some float panel".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(SHIFT, KeyCode::Char('S'))]),
                action: SYNC_INDEX.to_owned(),
                desc:   "sync question index (select tab and topic_tags tab)".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(CTRL, KeyCode::Char('p'))]),
                action: TOGGLE_MENU.to_owned(),
                desc:   "show or hide menu(only edit)".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(CTRL, KeyCode::Char('r'))]),
                action: RE_QS_DETAIL.to_owned(),
                desc:   "re get question detail (reference tab0/select cursor question info)"
                    .to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(SHIFT, KeyCode::Char('S'))]),
                action: SUBMIT_CODE.to_owned(),
                desc:   "submit this question code to leetcode (in edit show menu)".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(SHIFT, KeyCode::Char('T'))]),
                action: TEST_CODE.to_owned(),
                desc:   "test this question code to leetcode (in edit show menu)".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(CTRL, KeyCode::Char('t'))]),
                action: TOGGLE_TEST_RES.to_owned(),
                desc:   "show or hide test result (only tab1/edit)".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(CTRL, KeyCode::Char('s'))]),
                action: TOGGLE_SUBMIT_RES.to_owned(),
                desc:   "show or hide test result (only tab1/edit)".to_owned(),
            },
            KeyMap {
                keys:   Keys(vec![Key::new(NO_CONTROL, KeyCode::Enter)]),
                action: TOGGLE_CURSOR.to_owned(),
                desc:   "toggle cursor item(add or rm topic_tags, or goto tab1/edit)".to_owned(),
            },
        ]);

        Self { keymap }
    }
}
