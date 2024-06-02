use leetcode_api::leetcode::resps::run_res::RunResult;
use ratatui::widgets::ScrollbarState;

#[derive(Debug)]
#[derive(Default)]
pub struct SubmitState {
    pub show:    bool,
    pub content: RunResult,

    pub vert_scroll_state: ScrollbarState,
    pub vert_scroll:       usize,

    pub hori_scroll_state: ScrollbarState,
    pub hori_scroll:       usize,

    pub row_len: usize,

    pub need_add_test_case: bool,

    pub add_case_handle: Option<tokio::task::JoinHandle<()>>,
}

impl SubmitState {
    pub fn not_need_add(&mut self) {
        self.need_add_test_case = false;
    }
    pub const fn need_add(&self) -> bool {
        self.need_add_test_case
    }

    pub fn toggle(&mut self) {
        self.show = !self.show;
    }
    pub fn close(&mut self) {
        self.show = false;
    }
    pub fn open(&mut self) {
        self.show = true;
    }

    pub fn first(&mut self) {
        self.vert_scroll = 0;
        self.vert_scroll_state = self
            .vert_scroll_state
            .position(self.vert_scroll);
    }
    pub fn last(&mut self) {
        self.vert_scroll = self.row_len.saturating_sub(4);
        self.vert_scroll_state = self
            .vert_scroll_state
            .position(self.vert_scroll);
    }
    pub fn goto_head(&mut self) {
        self.hori_scroll = 0;
        self.hori_scroll_state = self
            .hori_scroll_state
            .position(self.hori_scroll);
    }

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
