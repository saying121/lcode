use ratatui::widgets::ScrollbarState;
use tui_textarea::TextArea;

use crate::leetcode::resps::run_res::RunResult;

use super::InputMode;

// tab1 edit
pub struct EditCode<'tab1> {
    pub code_block: TextArea<'tab1>,
    pub edit_code: bool,
    pub code_block_mode: InputMode,

    pub vertical_row_len: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_col_len: usize,
    pub horizontal_scroll_state: ScrollbarState,
    pub horizontal_scroll: usize,

    // test and submit
    pub submiting: bool,
    pub show_pop_menu: bool,

    pub submit_res: RunResult,
    pub show_submit_res: bool,
    pub submit_vert_scroll_state: ScrollbarState,
    pub submit_vert_scroll: usize,
    pub submit_hori_scroll_state: ScrollbarState,
    pub submit_hori_scroll: usize,
    pub submit_row_len: usize,

    pub test_res: RunResult,
    pub show_test_res: bool,
    pub test_vert_scroll_state: ScrollbarState,
    pub test_vert_scroll: usize,
    pub test_hori_scroll_state: ScrollbarState,
    pub test_hori_scroll: usize,
    pub test_row_len: usize,
}

impl<'tab1> EditCode<'tab1> {
    pub fn submit_done(&mut self, res: RunResult) {
        self.submit_res = res;
        self.show_submit_res = true;
        self.submiting = false;
    }
    pub fn test_done(&mut self, res: RunResult) {
        self.test_res = res;
        self.show_test_res = true;
        self.submiting = false;
    }
    pub fn toggle_menu(&mut self) {
        self.show_pop_menu = !self.show_pop_menu;
    }
    pub fn toggle_test_res(&mut self) {
        self.show_test_res = !self.show_test_res;
    }
    pub fn toggle_submit_res(&mut self) {
        self.show_submit_res = !self.show_submit_res;
    }
    pub fn start_edit_code(&mut self) {
        self.edit_code = true;
    }

    pub fn be_code_normal(&mut self) {
        self.code_block_mode = InputMode::Normal;
    }
}

impl<'tab1> EditCode<'tab1> {
    pub fn new() -> Self {
        Self {
            code_block: TextArea::default(),
            edit_code: false,
            code_block_mode: InputMode::Normal,

            horizontal_col_len: 0,
            horizontal_scroll: 0,
            horizontal_scroll_state: ScrollbarState::default(),
            vertical_row_len: 0,
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),

            // submit and test
            submiting: false,
            show_pop_menu: false,

            submit_res: RunResult::default(),
            show_submit_res: false,
            submit_vert_scroll_state: ScrollbarState::default(),
            submit_vert_scroll: 0,
            submit_hori_scroll_state: ScrollbarState::default(),
            submit_hori_scroll: 0,
            submit_row_len: 0,

            test_res: RunResult::default(),
            show_test_res: false,
            test_vert_scroll_state: ScrollbarState::default(),
            test_vert_scroll: 0,
            test_hori_scroll_state: ScrollbarState::default(),
            test_hori_scroll: 0,
            test_row_len: 0,
        }
    }

    pub fn close_pop(&mut self) {
        if self.show_test_res {
            self.show_test_res = false;
        } else if self.show_submit_res {
            self.show_submit_res = false;
        } else if self.show_pop_menu {
            self.show_pop_menu = false;
        }
    }

    pub fn vertical_scroll_j(&mut self) {
        if self.show_test_res {
            if self.test_vert_scroll < self.test_row_len.saturating_sub(4) {
                self.test_vert_scroll = self
                    .test_vert_scroll
                    .saturating_add(1);
                self.test_vert_scroll_state = self
                    .test_vert_scroll_state
                    .position(self.test_vert_scroll);
            }
        } else if self.show_submit_res
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
        } else if self.vertical_scroll
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
    }

    pub fn vertical_scroll_k(&mut self) {
        if self.show_test_res {
            self.test_vert_scroll = self
                .test_vert_scroll
                .saturating_sub(1);
            self.test_vert_scroll_state = self
                .test_vert_scroll_state
                .position(self.test_vert_scroll);
        } else if self.show_submit_res {
            self.submit_vert_scroll = self
                .submit_vert_scroll
                .saturating_sub(1);
            self.submit_vert_scroll_state = self
                .submit_vert_scroll_state
                .position(self.submit_vert_scroll);
        } else {
            self.vertical_scroll = self
                .vertical_scroll
                .saturating_sub(1);
            self.vertical_scroll_state = self
                .vertical_scroll_state
                .position(self.vertical_scroll);
        }
    }

    pub fn horizontal_scroll_h(&mut self) {
        if self.show_test_res {
            self.test_hori_scroll = self
                .test_hori_scroll
                .saturating_sub(2);
            self.test_hori_scroll_state = self
                .test_hori_scroll_state
                .position(self.test_hori_scroll);
        } else if self.show_submit_res {
            self.submit_hori_scroll = self
                .submit_hori_scroll
                .saturating_sub(2);
            self.submit_hori_scroll_state = self
                .submit_hori_scroll_state
                .position(self.submit_hori_scroll);
        } else {
            self.horizontal_scroll = self
                .horizontal_scroll
                .saturating_sub(1);
            self.horizontal_scroll_state = self
                .horizontal_scroll_state
                .position(self.horizontal_scroll);
        }
    }

    pub fn horizontal_scroll_l(&mut self) {
        if self.show_test_res {
            self.test_hori_scroll = self
                .test_hori_scroll
                .saturating_add(2);
            self.test_hori_scroll_state = self
                .test_hori_scroll_state
                .position(self.test_hori_scroll);
        } else if self.show_submit_res {
            self.submit_hori_scroll = self
                .submit_hori_scroll
                .saturating_add(2);
            self.submit_hori_scroll_state = self
                .submit_hori_scroll_state
                .position(self.submit_hori_scroll);
        } else {
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
    }

    pub fn vertical_scroll_gg(&mut self) {
        if self.show_submit_res {
            self.submit_vert_scroll = 0;
            self.submit_vert_scroll_state = self
                .submit_vert_scroll_state
                .position(self.submit_vert_scroll);
        } else if self.show_test_res {
            self.test_vert_scroll = 0;
            self.test_vert_scroll_state = self
                .test_vert_scroll_state
                .position(self.test_vert_scroll);
        } else {
            self.vertical_scroll = 0;
            self.vertical_scroll_state = self
                .vertical_scroll_state
                .position(self.vertical_scroll);
        }
    }

    #[allow(non_snake_case)]
    pub fn vertical_scroll_G(&mut self) {
        if self.show_submit_res {
            self.submit_vert_scroll = self
                .submit_row_len
                .saturating_sub(4);
            self.submit_vert_scroll_state = self
                .submit_vert_scroll_state
                .position(self.submit_vert_scroll);
        } else if self.show_test_res {
            self.test_vert_scroll = self.test_row_len.saturating_sub(4);
            self.test_vert_scroll_state = self
                .test_vert_scroll_state
                .position(self.test_vert_scroll);
        } else {
            self.vertical_scroll = self
                .vertical_row_len
                .saturating_sub(4);
            self.vertical_scroll_state = self
                .vertical_scroll_state
                .position(self.vertical_scroll);
        }
    }
    pub fn submit_res_view_head(&mut self) {
        self.submit_hori_scroll = 0;
        self.submit_hori_scroll_state = self
            .submit_hori_scroll_state
            .position(self.submit_hori_scroll);
    }
    pub fn test_res_view_head(&mut self) {
        self.test_hori_scroll = 0;
        self.test_hori_scroll_state = self
            .test_hori_scroll_state
            .position(self.test_hori_scroll);
    }
}
