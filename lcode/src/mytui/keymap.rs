use crossterm::event::KeyCode;
use key_parse::{self, keymap::*};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum TuiMode {
    /// input panel
    Normal,
    /// input panel
    Insert,
    /// input panel
    Select,

    /// not enter input
    #[default]
    OutEdit,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct KeyMap {
    pub keys: Keys,
    pub r#do: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TuiKeyMap {
    pub keymap: Vec<KeyMap>,
}
#[test]
fn feature() {
    let a = TuiKeyMap::default();
    let a = toml::to_string(&a).unwrap();
    println!("{}", a);
}
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

pub const SAVE_CODE:&str="save_code";

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
pub const GOTO_EDIT: &str = "goto_edit";
pub const SYNC_INDEX: &str = "sync_index";

pub const ESCAPE: &str = "escape";

impl Default for TuiKeyMap {
    fn default() -> Self {
        // if have float panel that first care
        let glob = vec![
            KeyMap {
                keys: Keys(vec![Key::new(NO_CONTROL, KeyCode::Tab)]),
                r#do: NEXT_TAB.to_owned(),
                desc: "next tab".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(NO_CONTROL, KeyCode::Right)]),
                r#do: NEXT_TAB.to_owned(),
                desc: "next tab".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(SHIFT, KeyCode::BackTab)]),
                r#do: PREV_TAB.to_owned(),
                desc: "prev tab".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(SHIFT, KeyCode::Left)]),
                r#do: PREV_TAB.to_owned(),
                desc: "prev tab".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(CTRL, KeyCode::Char('l'))]),
                r#do: REDRAW.to_owned(),
                desc: "redraw ui".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(CTRL, KeyCode::Char('q'))]),
                r#do: EXIT.to_owned(),
                desc: "exit lcode".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![
                    Key::new(NO_CONTROL, KeyCode::Char('g')),
                    Key::new(NO_CONTROL, KeyCode::Char('g')),
                ]),
                r#do: TOP.to_owned(),
                desc: "go to top".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(SHIFT, KeyCode::Char('G'))]),
                r#do: BOTTOM.to_owned(),
                desc: "go to bottom".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('o'))]),
                r#do: EDIT_CODE_EDITOR.to_owned(),
                desc: "edit code with your editor".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('e'))]),
                r#do: EDIT_IN_TUI.to_owned(),
                desc: "Enter input panel, like input line, code panel".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('l'))]),
                r#do: RIGHT.to_owned(),
                desc: "panel content move right".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('h'))]),
                r#do: LEFT.to_owned(),
                desc: "panel content move left".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('k'))]),
                r#do: UP.to_owned(),
                desc: "panel content move up".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(NO_CONTROL, KeyCode::Char('j'))]),
                r#do: DOWN.to_owned(),
                desc: "panel content move down".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(ALT, KeyCode::Char('h'))]),
                r#do: PANEL_LEFT.to_owned(),
                desc: "switch to left panel".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(ALT, KeyCode::Char('l'))]),
                r#do: PANEL_RIGHT.to_owned(),
                desc: "switch to right panel".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(ALT, KeyCode::Char('j'))]),
                r#do: PANEL_DOWN.to_owned(),
                desc: "switch to down panel".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(ALT, KeyCode::Char('k'))]),
                r#do: PANEL_UP.to_owned(),
                desc: "switch to up panel".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(SHIFT, KeyCode::Char('H'))]),
                r#do: HEAD.to_owned(),
                desc: "To the first column of the panel content.".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(SHIFT, KeyCode::Char('L'))]),
                r#do: TAIL.to_owned(),
                desc: "To the end column of the panel content.".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(NO_CONTROL, KeyCode::Esc)]),
                r#do: ESCAPE.to_owned(),
                desc: "close some float panel".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(SHIFT, KeyCode::Char('S'))]),
                r#do: SYNC_INDEX.to_owned(),
                desc: "sync question index (select tab and topic_tags tab)".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(CTRL, KeyCode::Char('p'))]),
                r#do: TOGGLE_MENU.to_owned(),
                desc: "show or hide menu(only edit)".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(NO_CONTROL, KeyCode::Enter)]),
                r#do: GOTO_EDIT.to_owned(),
                desc: "go to tab1".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(CTRL, KeyCode::Char('r'))]),
                r#do: RE_QS_DETAIL.to_owned(),
                desc: "re get question detail".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(SHIFT, KeyCode::Char('S'))]),
                r#do: SUBMIT_CODE.to_owned(),
                desc: "submit this question code to leetcode (in edit show menu)".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(SHIFT, KeyCode::Char('T'))]),
                r#do: TEST_CODE.to_owned(),
                desc: "test this question code to leetcode (in edit show menu)".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(CTRL, KeyCode::Char('t'))]),
                r#do: TOGGLE_TEST_RES.to_owned(),
                desc: "show or hide test result (only edit)".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(CTRL, KeyCode::Char('s'))]),
                r#do: TOGGLE_SUBMIT_RES.to_owned(),
                desc: "show or hide test result (only edit)".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(CTRL, KeyCode::Char('s'))]),
                r#do: SAVE_CODE.to_owned(),
                desc: "save tui code to file (only edit)".to_owned(),
            },
            KeyMap {
                keys: Keys(vec![Key::new(NO_CONTROL, KeyCode::Enter)]),
                r#do: TOGGLE_CURSOR.to_owned(),
                desc: "toggle cursor item".to_owned(),
            },
        ];
        Self {
            keymap: glob
        }
    }
}
