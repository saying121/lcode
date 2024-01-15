use crossterm::event::{self, Event as CrossEvent, KeyCode};
use leetcode_api::leetcode::resps::run_res::RunResult;
use ratatui::widgets::ScrollbarState;
use tui_textarea::{CursorMove, Input, Key, Scrolling, TextArea};

use super::TuiMode;

// tab1 edit
#[derive(Default)]
pub struct EditCode<'tab1> {
    pub code_block:      TextArea<'tab1>,
    pub code_block_mode: TuiMode,

    pub vertical_row_len:        usize,
    pub vertical_scroll_state:   ScrollbarState,
    pub vertical_scroll:         usize,
    pub horizontal_col_len:      usize,
    pub horizontal_scroll_state: ScrollbarState,
    pub horizontal_scroll:       usize,

    // test and submit
    pub submitting:    bool,
    pub show_pop_menu: bool,

    pub submit_res:               RunResult,
    pub show_submit_res:          bool,
    pub submit_vert_scroll_state: ScrollbarState,
    pub submit_vert_scroll:       usize,
    pub submit_hori_scroll_state: ScrollbarState,
    pub submit_hori_scroll:       usize,
    pub submit_row_len:           usize,

    pub test_res:               RunResult,
    pub show_test_res:          bool,
    pub test_vert_scroll_state: ScrollbarState,
    pub test_vert_scroll:       usize,
    pub test_hori_scroll_state: ScrollbarState,
    pub test_hori_scroll:       usize,
    pub test_row_len:           usize,
}

impl<'tab1> EditCode<'tab1> {
    pub fn normal_map(&mut self, event: CrossEvent) -> bool {
        match event.into() {
            // Mappings in normal mode
            Input {
                key: Key::Char('d'), ctrl: false, ..
            } => match event::read() {
                Ok(CrossEvent::Key(keyevent)) => match keyevent.code {
                    KeyCode::Char('d') => {
                        self.code_block
                            .move_cursor(CursorMove::Head);
                        self.code_block
                            .delete_line_by_end();
                        self.code_block.delete_next_char()
                    },
                    KeyCode::Char('w') => self.code_block.delete_next_word(),
                    _ => false,
                },
                _ => false,
            },
            Input { key: Key::Char('g'), .. } => match event::read() {
                Ok(CrossEvent::Key(key)) => {
                    if key.code == KeyCode::Char('g') {
                        self.code_block
                            .move_cursor(CursorMove::Top);
                        true
                    }
                    else {
                        false
                    }
                },
                _ => false,
            },
            Input { key: Key::Char('G'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::Bottom);
                true
            },
            Input { key: Key::Char('h'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::Back);
                true
            },
            Input { key: Key::Char('j'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::Down);
                true
            },
            Input { key: Key::Char('k'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::Up);
                true
            },
            Input { key: Key::Char('l'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::Forward);
                true
            },
            Input { key: Key::Char('w'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::WordForward);
                true
            },
            Input {
                key: Key::Char('b'), ctrl: false, ..
            } => {
                self.code_block
                    .move_cursor(CursorMove::WordBack);
                true
            },
            Input { key: Key::Char('^' | '0'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::Head);
                true
            },
            Input { key: Key::Char('$'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::End);
                true
            },
            Input { key: Key::Char('D'), .. } => self
                .code_block
                .delete_line_by_end(),
            Input { key: Key::Char('C'), .. } => {
                self.code_block
                    .delete_line_by_end();
                self.be_code_insert()
            },
            Input { key: Key::Char('p'), .. } => self.code_block.paste(),
            Input {
                key: Key::Char('u'), ctrl: false, ..
            } => self.code_block.undo(),
            Input { key: Key::Char('r'), ctrl: true, .. } => self.code_block.redo(),
            Input { key: Key::Char('x'), .. } => self.code_block.delete_next_char(),
            Input { key: Key::Char('i'), .. } => self.be_code_insert(),
            Input { key: Key::Char('a'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::Forward);
                self.be_code_insert()
            },
            Input { key: Key::Char('A'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::End);
                self.be_code_insert()
            },
            Input { key: Key::Char('o'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::End);
                self.code_block.insert_newline();
                self.be_code_insert()
            },
            Input { key: Key::Char('O'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::Head);
                self.code_block.insert_newline();
                self.code_block
                    .move_cursor(CursorMove::Up);
                self.be_code_insert()
            },
            Input { key: Key::Char('I'), .. } => {
                self.code_block
                    .move_cursor(CursorMove::Head);
                self.be_code_insert()
            },
            Input { key: Key::Char('e'), ctrl: true, .. } => {
                self.code_block.scroll((1, 0));
                true
            },
            Input { key: Key::Char('y'), ctrl: true, .. } => {
                self.code_block.scroll((-1, 0));
                true
            },
            Input { key: Key::Char('d'), ctrl: true, .. } => {
                self.code_block
                    .scroll(Scrolling::HalfPageDown);
                true
            },
            Input { key: Key::Char('u'), ctrl: true, .. } => {
                self.code_block
                    .scroll(Scrolling::HalfPageUp);
                true
            },
            Input { key: Key::Char('f'), ctrl: true, .. } => {
                self.code_block
                    .scroll(Scrolling::PageDown);
                true
            },
            Input { key: Key::Char('b'), ctrl: true, .. } => {
                self.code_block
                    .scroll(Scrolling::PageUp);
                true
            },

            Input { key: Key::Char('q'), .. } => self.quit_edit_tui(),
            _ => false,
        }
    }

    pub fn insert_keymap(&mut self, event: CrossEvent) -> bool {
        match event.into() {
            Input { key: Key::Esc, .. } => self.be_code_normal(),
            input => self.code_block.input(input), // Use default key mappings in insert mode(emacs)
        }
    }
    pub fn quit_edit_tui(&mut self) -> bool {
        self.code_block_mode = TuiMode::OutEdit;
        true
    }
    pub fn be_code_insert(&mut self) -> bool {
        self.code_block_mode = TuiMode::Insert;
        true
    }

    pub fn be_code_normal(&mut self) -> bool {
        self.code_block_mode = TuiMode::Normal;
        true
    }
    pub fn start_edit_tui(&mut self) -> bool {
        self.code_block_mode = TuiMode::Normal;
        true
    }
}

impl<'tab1> EditCode<'tab1> {
    pub fn toggle_menu(&mut self) -> bool {
        self.show_pop_menu = !self.show_pop_menu;
        true
    }
    pub fn toggle_test_res(&mut self) -> bool {
        self.show_test_res = !self.show_test_res;
        true
    }
    pub fn toggle_submit_res(&mut self) -> bool {
        self.show_submit_res = !self.show_submit_res;
        true
    }
}

impl<'tab1> EditCode<'tab1> {
    pub fn new() -> Self {
        Self {
            code_block:      TextArea::default(),
            code_block_mode: TuiMode::OutEdit,

            horizontal_col_len:      0,
            horizontal_scroll:       0,
            horizontal_scroll_state: ScrollbarState::default(),
            vertical_row_len:        0,
            vertical_scroll:         0,
            vertical_scroll_state:   ScrollbarState::default(),

            // submit and test
            submitting:    false,
            show_pop_menu: false,

            submit_res:               RunResult::default(),
            show_submit_res:          false,
            submit_vert_scroll_state: ScrollbarState::default(),
            submit_vert_scroll:       0,
            submit_hori_scroll_state: ScrollbarState::default(),
            submit_hori_scroll:       0,
            submit_row_len:           0,

            test_res:               RunResult::default(),
            show_test_res:          false,
            test_vert_scroll_state: ScrollbarState::default(),
            test_vert_scroll:       0,
            test_hori_scroll_state: ScrollbarState::default(),
            test_hori_scroll:       0,
            test_row_len:           0,
        }
    }

    pub fn close_pop(&mut self) -> bool {
        if self.show_test_res {
            self.show_test_res = false;
        }
        else if self.show_submit_res {
            self.show_submit_res = false;
        }
        else if self.show_pop_menu {
            self.show_pop_menu = false;
        }
        true
    }

    pub fn vertical_scroll_j(&mut self) -> bool {
        if self.show_test_res {
            if self.test_vert_scroll < self.test_row_len.saturating_sub(4) {
                self.test_vert_scroll = self
                    .test_vert_scroll
                    .saturating_add(1);
                self.test_vert_scroll_state = self
                    .test_vert_scroll_state
                    .position(self.test_vert_scroll);
            }
        }
        else if self.show_submit_res
            && self.submit_vert_scroll
                < self
                    .submit_row_len
                    .saturating_sub(4)
        {
            self.submit_vert_scroll = self
                .submit_vert_scroll
                .saturating_add(1);
            self.submit_vert_scroll_state = self
                .submit_vert_scroll_state
                .position(self.submit_vert_scroll);
        }
        else if self.vertical_scroll
            < self
                .vertical_row_len
                .saturating_sub(4)
        {
            self.vertical_scroll = self
                .vertical_scroll
                .saturating_add(1);
            self.vertical_scroll_state = self
                .vertical_scroll_state
                .position(self.vertical_scroll);
        }
        true
    }

    pub fn vertical_scroll_k(&mut self) -> bool {
        if self.show_test_res {
            self.test_vert_scroll = self
                .test_vert_scroll
                .saturating_sub(1);
            self.test_vert_scroll_state = self
                .test_vert_scroll_state
                .position(self.test_vert_scroll);
        }
        else if self.show_submit_res {
            self.submit_vert_scroll = self
                .submit_vert_scroll
                .saturating_sub(1);
            self.submit_vert_scroll_state = self
                .submit_vert_scroll_state
                .position(self.submit_vert_scroll);
        }
        else {
            self.vertical_scroll = self
                .vertical_scroll
                .saturating_sub(1);
            self.vertical_scroll_state = self
                .vertical_scroll_state
                .position(self.vertical_scroll);
        }
        true
    }

    pub fn horizontal_scroll_h(&mut self) -> bool {
        if self.show_test_res {
            self.test_hori_scroll = self
                .test_hori_scroll
                .saturating_sub(2);
            self.test_hori_scroll_state = self
                .test_hori_scroll_state
                .position(self.test_hori_scroll);
        }
        else if self.show_submit_res {
            self.submit_hori_scroll = self
                .submit_hori_scroll
                .saturating_sub(2);
            self.submit_hori_scroll_state = self
                .submit_hori_scroll_state
                .position(self.submit_hori_scroll);
        }
        else {
            self.horizontal_scroll = self
                .horizontal_scroll
                .saturating_sub(1);
            self.horizontal_scroll_state = self
                .horizontal_scroll_state
                .position(self.horizontal_scroll);
        }
        true
    }

    pub fn horizontal_scroll_l(&mut self) -> bool {
        if self.show_test_res {
            self.test_hori_scroll = self
                .test_hori_scroll
                .saturating_add(2);
            self.test_hori_scroll_state = self
                .test_hori_scroll_state
                .position(self.test_hori_scroll);
        }
        else if self.show_submit_res {
            self.submit_hori_scroll = self
                .submit_hori_scroll
                .saturating_add(2);
            self.submit_hori_scroll_state = self
                .submit_hori_scroll_state
                .position(self.submit_hori_scroll);
        }
        else {
            if self.horizontal_scroll
                < self
                    .horizontal_col_len
                    .saturating_sub(4)
            {
                self.horizontal_scroll = self
                    .horizontal_scroll
                    .saturating_add(1);
            }
            self.horizontal_scroll_state = self
                .horizontal_scroll_state
                .position(self.horizontal_scroll);
        }
        true
    }

    pub fn vertical_scroll_gg(&mut self) -> bool {
        if self.show_submit_res {
            self.submit_vert_scroll = 0;
            self.submit_vert_scroll_state = self
                .submit_vert_scroll_state
                .position(self.submit_vert_scroll);
        }
        else if self.show_test_res {
            self.test_vert_scroll = 0;
            self.test_vert_scroll_state = self
                .test_vert_scroll_state
                .position(self.test_vert_scroll);
        }
        else {
            self.vertical_scroll = 0;
            self.vertical_scroll_state = self
                .vertical_scroll_state
                .position(self.vertical_scroll);
        }
        true
    }

    #[allow(non_snake_case)]
    pub fn vertical_scroll_G(&mut self) -> bool {
        if self.show_submit_res {
            self.submit_vert_scroll = self
                .submit_row_len
                .saturating_sub(4);
            self.submit_vert_scroll_state = self
                .submit_vert_scroll_state
                .position(self.submit_vert_scroll);
        }
        else if self.show_test_res {
            self.test_vert_scroll = self.test_row_len.saturating_sub(4);
            self.test_vert_scroll_state = self
                .test_vert_scroll_state
                .position(self.test_vert_scroll);
        }
        else {
            self.vertical_scroll = self
                .vertical_row_len
                .saturating_sub(4);
            self.vertical_scroll_state = self
                .vertical_scroll_state
                .position(self.vertical_scroll);
        }
        true
    }
    fn submit_res_view_head(&mut self) {
        self.submit_hori_scroll = 0;
        self.submit_hori_scroll_state = self
            .submit_hori_scroll_state
            .position(self.submit_hori_scroll);
    }
    fn test_res_view_head(&mut self) {
        self.test_hori_scroll = 0;
        self.test_hori_scroll_state = self
            .test_hori_scroll_state
            .position(self.test_hori_scroll);
    }
    pub fn pop_head(&mut self) -> bool {
        if self.show_submit_res {
            self.submit_res_view_head();
        }
        if self.show_test_res {
            self.test_res_view_head();
        }
        true
    }
}
