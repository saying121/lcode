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

    (ESCAPE,            "escape")
);

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq)]
#[derive(Serialize, Deserialize)]
pub struct KeyMap {
    pub keys:   Keys,
    pub action: String,
    #[serde(default)]
    pub desc:   String,
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
    pub keymap: HashSet<KeyMap>,
}

impl TuiKeyMap {
    /// Add extra keymap
    pub fn add_keymap(&mut self, add: HashSet<KeyMap>) {
        for ele in &add {
            self.keymap.remove(ele);
        }
        self.keymap.extend(add);
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

impl Default for TuiKeyMap {
    fn default() -> Self {
        let mps = keymaps!(
            (vec![Key::new(NO_CONTROL, KeyCode::Char('g')), Key::new(NO_CONTROL, KeyCode::Char('g'))],  TOP,  "Go to top");
            (vec![Key::new(SHIFT, KeyCode::Char('G'))],      BOTTOM,               "Go to bottom");

            (vec![Key::new(SHIFT, KeyCode::Char('H'))],      HEAD,               "To the first column of the panel content.");
            (vec![Key::new(SHIFT, KeyCode::Char('L'))],      TAIL,               "To the last column of the panel content.");

            (vec![Key::new(NO_CONTROL, KeyCode::Char('k'))], UP,                 "Panel content move up");
            (vec![Key::new(NO_CONTROL, KeyCode::Char('j'))], DOWN,               "Panel content move down");
            (vec![Key::new(NO_CONTROL, KeyCode::Char('h'))], LEFT,               "Panel content move left");
            (vec![Key::new(NO_CONTROL, KeyCode::Char('l'))], RIGHT,              "Panel content move right");

            (vec![Key::new(ALT, KeyCode::Char('k'))],        PANEL_UP,           "Switch to up panel(topic tags)");
            (vec![Key::new(ALT, KeyCode::Char('j'))],        PANEL_DOWN,         "Switch to down panel(topic tags)");
            (vec![Key::new(ALT, KeyCode::Char('h'))],        PANEL_LEFT,         "Switch to left panel(topic tags)");
            (vec![Key::new(ALT, KeyCode::Char('l'))],        PANEL_RIGHT,        "Switch to right panel(topic tags)");

            (vec![Key::new(NO_CONTROL, KeyCode::Tab)],       NEXT_TAB,           "Next tab");
            (vec![Key::new(SHIFT, KeyCode::BackTab)],        PREV_TAB,           "Prev tab");

            (vec![Key::new(NO_CONTROL, KeyCode::Char('o'))], EDIT_CODE_EDITOR,   "Edit cursor question(or current question) with your editor");
            (vec![Key::new(NO_CONTROL, KeyCode::Char('e'))], EDIT_IN_TUI,        "Enter input block");

            (vec![Key::new(NO_CONTROL, KeyCode::Esc)],       ESCAPE,             "Close some float panel");
            (vec![Key::new(CTRL, KeyCode::Char('l'))],       REDRAW,             "Redraw ui");
            (vec![Key::new(CTRL, KeyCode::Char('q'))],       EXIT,               "Exit lcode");
            (vec![Key::new(SHIFT, KeyCode::Char('S'))],      SYNC_INDEX,         "Sync question index (select tab and topic_tags tab)");
            (vec![Key::new(CTRL, KeyCode::Char('r'))],       RE_QS_DETAIL,       "Re get question detail (reference tab0/select cursor question info)");

            (vec![Key::new(CTRL, KeyCode::Char('p'))],       TOGGLE_MENU,        "Show or hide menu(only edit)");
            (vec![Key::new(CTRL, KeyCode::Char('t'))],       TOGGLE_TEST_RES,    "Show or hide test result (only tab1/edit)");
            (vec![Key::new(CTRL, KeyCode::Char('s'))],       TOGGLE_SUBMIT_RES,  "Show or hide submit result (only tab1/edit)");
            (vec![Key::new(NO_CONTROL, KeyCode::Enter)],     TOGGLE_CURSOR,      "Trigger cursor item, in edit pop menu will active button (add or rm topic_tags, or goto tab1/edit)")

        );
        let keymap = HashSet::from(mps);

        Self { keymap }
    }
}
