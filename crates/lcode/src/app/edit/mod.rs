mod cmds;

use crossterm::event::{self, Event as CrossEvent, KeyCode};
use leetcode_api::leetcode::resps::run_res::RunResult;
use ratatui::widgets::ScrollbarState;
use tui_textarea::{CursorMove, Input, Key, Scrolling, TextArea};

use super::TuiMode;
use crate::mytui::my_widget::botton::{ButtonState, ButtonStates};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
pub struct ContentState {
    pub content_row_num:         usize,
    pub vert_scroll_state:       ScrollbarState,
    pub vert_scroll:             usize,
    pub column_len:              usize,
    pub horizontal_scroll_state: ScrollbarState,
    pub horizontal_scroll:       usize,
}

impl ContentState {
    pub fn up(&mut self) {
        self.vert_scroll = self.vert_scroll.saturating_sub(1);
        self.vert_scroll_state = self
            .vert_scroll_state
            .position(self.vert_scroll);
    }
    pub fn down(&mut self) {
        if self.vert_scroll < self.content_row_num.saturating_sub(4) {
            self.vert_scroll = self.vert_scroll.saturating_add(1);
            self.vert_scroll_state = self
                .vert_scroll_state
                .position(self.vert_scroll);
        }
    }
    pub fn left(&mut self) {
        self.horizontal_scroll = self.horizontal_scroll.saturating_sub(1);
        self.horizontal_scroll_state = self
            .horizontal_scroll_state
            .position(self.horizontal_scroll);
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct ButState {
    pub button_state:  ButtonStates,
    pub select_button: usize,
    pub show:          bool,
}

impl ButState {
    pub fn open(&mut self) {
        self.show = true;
    }
    pub fn close(&mut self) {
        self.show = false;
    }
    pub fn toggle(&mut self) {
        self.show = !self.show;
    }
    pub fn left(&mut self) {
        if self.button_state.states[self.select_button] != ButtonState::Active {
            self.button_state.states[self.select_button] = ButtonState::Normal;
        }
        self.select_button = self.select_button.saturating_sub(1);
        if self.button_state.states[self.select_button] != ButtonState::Active {
            self.button_state.states[self.select_button] = ButtonState::Selected;
        }
    }
    pub fn right(&mut self) {
        if self.button_state.states[self.select_button] != ButtonState::Active {
            self.button_state.states[self.select_button] = ButtonState::Normal;
        }

        self.select_button = self
            .select_button
            .saturating_add(1)
            .min(1);

        if self.button_state.states[self.select_button] != ButtonState::Active {
            self.button_state.states[self.select_button] = ButtonState::Selected;
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct SubmitState {
    pub result:            RunResult,
    pub show:              bool,
    pub vert_scroll_state: ScrollbarState,
    pub vert_scroll:       usize,
    pub hori_scroll_state: ScrollbarState,
    pub hori_scroll:       usize,
    pub row_len:           usize,
}

impl SubmitState {
    pub fn up(&mut self) {
        self.vert_scroll = self.vert_scroll.saturating_sub(1);
        self.vert_scroll_state = self
            .vert_scroll_state
            .position(self.vert_scroll);
    }
    pub fn down(&mut self) {
        if self.vert_scroll < self.row_len.saturating_sub(4) {
            self.vert_scroll = self.vert_scroll.saturating_add(1);
            self.vert_scroll_state = self
                .vert_scroll_state
                .position(self.vert_scroll);
        }
    }
    pub fn left(&mut self) {
        self.hori_scroll = self.hori_scroll.saturating_sub(2);
        self.hori_scroll_state = self
            .hori_scroll_state
            .position(self.hori_scroll);
    }
    pub fn right(&mut self) {
        self.hori_scroll = self.hori_scroll.saturating_add(2);
        self.hori_scroll_state = self
            .hori_scroll_state
            .position(self.hori_scroll);
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct TestState {
    pub result:            RunResult,
    pub show:              bool,
    pub vert_scroll_state: ScrollbarState,
    pub vert_scroll:       usize,
    pub hori_scroll_state: ScrollbarState,
    pub hori_scroll:       usize,
    pub row_len:           usize,
}

impl TestState {
    pub fn up(&mut self) {
        self.vert_scroll = self.vert_scroll.saturating_sub(1);
        self.vert_scroll_state = self
            .vert_scroll_state
            .position(self.vert_scroll);
    }
    pub fn down(&mut self) {
        if self.vert_scroll < self.row_len.saturating_sub(4) {
            self.vert_scroll = self.vert_scroll.saturating_add(1);
            self.vert_scroll_state = self
                .vert_scroll_state
                .position(self.vert_scroll);
        }
    }
    pub fn left(&mut self) {
        self.hori_scroll = self.hori_scroll.saturating_sub(2);
        self.hori_scroll_state = self
            .hori_scroll_state
            .position(self.hori_scroll);
    }
    pub fn right(&mut self) {
        self.hori_scroll = self.hori_scroll.saturating_add(2);
        self.hori_scroll_state = self
            .hori_scroll_state
            .position(self.hori_scroll);
    }
}

// tab1 edit
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct EditCode<'tab1> {
    pub code_block:      TextArea<'tab1>,
    pub code_block_mode: TuiMode,

    pub content_state: ContentState,

    pub submitting: bool,

    // pub show_pop_menu: bool,
    pub but_state: ButState,

    pub submit_state: SubmitState,
    pub test_state:   TestState,
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
                        self.code_block.delete_line_by_end();
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
            Input { key: Key::Char('D'), .. } => self.code_block.delete_line_by_end(),
            Input { key: Key::Char('C'), .. } => {
                self.code_block.delete_line_by_end();
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

// Show only one pop view every time.
impl<'tab1> EditCode<'tab1> {
    pub fn toggle_menu(&mut self) -> bool {
        self.but_state.toggle();
        self.test_state.show = false;
        self.submit_state.show = false;
        true
    }
    pub fn toggle_test_res(&mut self) -> bool {
        self.test_state.show = !self.test_state.show;
        self.but_state.close();
        self.submit_state.show = false;
        true
    }
    pub fn toggle_submit_res(&mut self) -> bool {
        self.submit_state.show = !self.submit_state.show;
        self.test_state.show = false;
        self.but_state.close();
        true
    }
}

impl<'tab1> EditCode<'tab1> {
    pub fn close_pop(&mut self) -> bool {
        if self.test_state.show {
            self.test_state.show = false;
        }
        else if self.submit_state.show {
            self.submit_state.show = false;
        }
        else if self.but_state.show {
            self.but_state.close();
        }
        true
    }

    pub fn vertical_scroll_j(&mut self) -> bool {
        if self.test_state.show {
            self.test_state.down();
        }
        else if self.submit_state.show {
            self.submit_state.down();
        }
        else if !self.but_state.show {
            self.content_state.down();
        }
        true
    }

    pub fn vertical_scroll_k(&mut self) -> bool {
        if self.test_state.show {
            self.test_state.up();
        }
        else if self.submit_state.show {
            self.submit_state.up();
        }
        else if !self.but_state.show {
            self.content_state.up();
        }
        true
    }

    pub fn horizontal_scroll_h(&mut self) -> bool {
        if self.test_state.show {
            self.test_state.left();
        }
        else if self.submit_state.show {
            self.submit_state.left();
        }
        else if self.but_state.show {
            self.but_state.left();
        }
        else {
            self.content_state.left();
        }
        true
    }

    pub fn horizontal_scroll_l(&mut self) -> bool {
        if self.test_state.show {
            self.test_state.right();
        }
        else if self.submit_state.show {
            self.submit_state.right();
        }
        else if self.but_state.show {
            self.but_state.right();
        }
        else {
            if self.content_state.horizontal_scroll
                < self
                    .content_state
                    .column_len
                    .saturating_sub(4)
            {
                self.content_state.horizontal_scroll = self
                    .content_state
                    .horizontal_scroll
                    .saturating_add(1);
            }
            self.content_state
                .horizontal_scroll_state = self
                .content_state
                .horizontal_scroll_state
                .position(self.content_state.horizontal_scroll);
        }
        true
    }

    pub fn vertical_scroll_gg(&mut self) -> bool {
        if self.submit_state.show {
            self.submit_state.vert_scroll = 0;
            self.submit_state.vert_scroll_state = self
                .submit_state
                .vert_scroll_state
                .position(self.submit_state.vert_scroll);
        }
        else if self.test_state.show {
            self.test_state.vert_scroll = 0;
            self.test_state.vert_scroll_state = self
                .test_state
                .vert_scroll_state
                .position(self.test_state.vert_scroll);
        }
        else {
            self.content_state.vert_scroll = 0;
            self.content_state.vert_scroll_state = self
                .content_state
                .vert_scroll_state
                .position(self.content_state.vert_scroll);
        }
        true
    }

    #[allow(non_snake_case)]
    pub fn vertical_scroll_G(&mut self) -> bool {
        if self.submit_state.show {
            self.submit_state.vert_scroll = self
                .submit_state
                .row_len
                .saturating_sub(4);
            self.submit_state.vert_scroll_state = self
                .submit_state
                .vert_scroll_state
                .position(self.submit_state.vert_scroll);
        }
        else if self.test_state.show {
            self.test_state.vert_scroll = self
                .test_state
                .row_len
                .saturating_sub(4);
            self.test_state.vert_scroll_state = self
                .test_state
                .vert_scroll_state
                .position(self.test_state.vert_scroll);
        }
        else {
            self.content_state.vert_scroll = self
                .content_state
                .content_row_num
                .saturating_sub(4);
            self.content_state.vert_scroll_state = self
                .content_state
                .vert_scroll_state
                .position(self.content_state.vert_scroll);
        }
        true
    }
    fn submit_res_view_head(&mut self) {
        self.submit_state.hori_scroll = 0;
        self.submit_state.hori_scroll_state = self
            .submit_state
            .hori_scroll_state
            .position(self.submit_state.hori_scroll);
    }
    fn test_res_view_head(&mut self) {
        self.test_state.hori_scroll = 0;
        self.test_state.hori_scroll_state = self
            .test_state
            .hori_scroll_state
            .position(self.test_state.hori_scroll);
    }
    /// goto first column
    pub fn goto_pop_head(&mut self) -> bool {
        if self.submit_state.show {
            self.submit_res_view_head();
        }
        if self.test_state.show {
            self.test_res_view_head();
        }
        true
    }
}
