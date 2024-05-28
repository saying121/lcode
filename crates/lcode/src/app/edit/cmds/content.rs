use ratatui::widgets::ScrollbarState;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
pub struct ContentState {
    pub content_row_num: usize,

    pub vert_scroll_state: ScrollbarState,
    pub vert_scroll:       usize,

    pub column_len: usize,

    pub horizontal_scroll_state: ScrollbarState,
    pub horizontal_scroll:       usize,
}

impl ContentState {
    pub fn top(&mut self) {
        self.vert_scroll = 0;
        self.vert_scroll_state = self
            .vert_scroll_state
            .position(self.vert_scroll);
    }
    pub fn bottom(&mut self) {
        self.vert_scroll = self.content_row_num.saturating_sub(4);
        self.vert_scroll_state = self
            .vert_scroll_state
            .position(self.vert_scroll);
    }

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
    pub fn right(&mut self) {
        if self.horizontal_scroll < self.column_len.saturating_sub(4) {
            self.horizontal_scroll = self.horizontal_scroll.saturating_add(1);
        }
        self.horizontal_scroll_state = self
            .horizontal_scroll_state
            .position(self.horizontal_scroll);
    }
}
