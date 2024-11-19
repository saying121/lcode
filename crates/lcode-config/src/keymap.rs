use std::{collections::HashSet, fmt, hash::Hash};

use crossterm::event::KeyCode;
use key_parse::keymap::*;
use serde::{Deserialize, Serialize};

// toml can't serialize enum
macro_rules! actions {
    ($( ($key:ident, $val:literal) ); *) => {
        $(pub const $key: &str = $val;)*
    };
}
actions!(
    (PANEL_UP,          "panel_up");
    (PANEL_DOWN,        "panel_down");
    (PANEL_RIGHT,       "panel_right");
    (PANEL_LEFT,        "panel_left");

    (UP,                "up");
    (DOWN,              "down");
    (RIGHT,             "right");
    (LEFT,              "left");

    (TOGGLE_CURSOR,     "toggle");

    (TOP,               "top");
    (BOTTOM,            "bottom");

    (REDRAW,            "redraw");
    (EXIT,              "exit");

    (EDIT_CODE_EDITOR,  "edit_code");
    (EDIT_IN_TUI,       "edit_code_tui");

    (TOGGLE_SUBMIT_RES, "toggle_submit_res");
    (TOGGLE_TEST_RES,   "toggle_test_res");
    (TOGGLE_MENU,       "toggle_menu");

    (RE_QS_DETAIL,      "re_get_qs");

    (NEXT_TAB,          "next_tab");
    (PREV_TAB,          "prev_tab");

    (HEAD,              "head");
    (TAIL,              "tail");

    (SYNC_INDEX,        "sync_index");

    (ESCAPE,            "escape");

    (ADD_TEST_CASE,     "add_test_case")
);

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq)]
#[derive(Serialize, Deserialize)]
pub struct KeyMap {
    pub keys: Keys,
    pub action: String,
    #[serde(default)]
    pub desc: String,
}

impl fmt::Display for KeyMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = toml::to_string(self).unwrap_or_else(|_| "unknown keymap\n\n".to_owned());
        let mut a = res.split('\n');
        format!(
            "{:20}, {:30}, {}",
            a.next().unwrap_or_default(),
            a.next().unwrap_or_default(),
            a.next().unwrap_or_default()
        )
        .fmt(f)
    }
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

#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
#[derive(Debug)]
pub struct TuiKeyMap {
    #[serde(default)]
    #[serde(rename = "keymap")]
    pub map_set: HashSet<KeyMap>,
}

impl TuiKeyMap {
    /// Add extra keymap
    pub fn add_keymap(&mut self, add: HashSet<KeyMap>) {
        for ele in add {
            self.map_set.replace(ele);
        }
    }
}

macro_rules! keymaps {
    ($( ($keys:expr, $action:expr, $desc:literal) ); *) => {
        [
            $(
                KeyMap {
                    keys:   Keys($keys),
                    action: $action.to_owned(),
                    desc:   $desc.to_owned(),
                },
            )*
        ]
    };
}

const fn kc(ch: char) -> KeyCode {
    KeyCode::Char(ch)
}

impl Default for TuiKeyMap {
    fn default() -> Self {
        let (tab, backtab) = (KeyCode::Tab, KeyCode::BackTab);
        let esc = KeyCode::Esc;
        let enter = KeyCode::Enter;

        let g = Key::new(NO_CONTROL, kc('g'));

        let mps = keymaps!(
            (vec![g, g],                          TOP,               "Go to top");
            (vec![Key::new(SHIFT,      kc('G'))], BOTTOM,            "Go to bottom");

            (vec![Key::new(SHIFT,      kc('H'))], HEAD,              "To the first column of the panel content.");
            (vec![Key::new(SHIFT,      kc('L'))], TAIL,              "To the last column of the panel content.");

            (vec![Key::new(NO_CONTROL, kc('h'))], LEFT,              "Panel content move left");
            (vec![Key::new(NO_CONTROL, kc('j'))], DOWN,              "Panel content move down");
            (vec![Key::new(NO_CONTROL, kc('k'))], UP,                "Panel content move up");
            (vec![Key::new(NO_CONTROL, kc('l'))], RIGHT,             "Panel content move right");

            (vec![Key::new(ALT,        kc('h'))], PANEL_LEFT,        "Switch to left panel(topic tags)");
            (vec![Key::new(ALT,        kc('j'))], PANEL_DOWN,        "Switch to down panel(topic tags)");
            (vec![Key::new(ALT,        kc('k'))], PANEL_UP,          "Switch to up panel(topic tags)");
            (vec![Key::new(ALT,        kc('l'))], PANEL_RIGHT,       "Switch to right panel(topic tags)");

            (vec![Key::new(NO_CONTROL,     tab)], NEXT_TAB,          "Next tab");
            (vec![Key::new(SHIFT,      backtab)], PREV_TAB,          "Prev tab");

            (vec![Key::new(NO_CONTROL, kc('o'))], EDIT_CODE_EDITOR,  "Edit cursor question(or current question) with your editor");
            (vec![Key::new(NO_CONTROL, kc('e'))], EDIT_IN_TUI,       "Enter input block");

            (vec![Key::new(NO_CONTROL,     esc)], ESCAPE,            "Close some float panel");
            (vec![Key::new(CTRL,       kc('l'))], REDRAW,            "Redraw ui");
            (vec![Key::new(CTRL,       kc('q'))], EXIT,              "Exit lcode");
            (vec![Key::new(SHIFT,      kc('S'))], SYNC_INDEX,        "Sync question index (select tab and topic_tags tab)");
            (vec![Key::new(CTRL,       kc('r'))], RE_QS_DETAIL,      "Re get question detail (reference tab0/select cursor question info)");

            (vec![Key::new(CTRL,       kc('p'))], TOGGLE_MENU,       "Show or hide menu(only edit)");
            (vec![Key::new(CTRL,       kc('t'))], TOGGLE_TEST_RES,   "Show or hide test result (only tab1/edit)");
            (vec![Key::new(CTRL,       kc('s'))], TOGGLE_SUBMIT_RES, "Show or hide submit result (only tab1/edit)");
            (vec![Key::new(NO_CONTROL,   enter)], TOGGLE_CURSOR,     "Trigger cursor item, in edit pop menu will active button (add or rm topic_tags, or goto tab1/edit)");

            (vec![Key::new(NO_CONTROL, kc('a'))], ADD_TEST_CASE,     "When submit error can add test case. (tab1/edit)")

        );
        let keymap = HashSet::from(mps);

        Self { map_set: keymap }
    }
}
