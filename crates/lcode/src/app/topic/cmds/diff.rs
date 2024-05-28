use ratatui::widgets::ListState;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct DiffState {
    pub user_diff:    String,
    pub difficulties: Box<[String]>,
    pub list_state:   ListState,
}

impl DiffState {
    pub fn new(difficulties: Box<[String]>) -> Self {
        Self {
            user_diff: String::new(),
            difficulties,
            list_state: ListState::default(),
        }
    }
    pub fn toggle_diff(&mut self) {
        let index = self
            .list_state
            .selected()
            .unwrap_or_default();
        let diff = self
            .difficulties
            .get(index)
            .expect("get difficulty failed");
        if self.user_diff == *diff {
            self.user_diff = String::new();
        }
        else {
            self.user_diff.clone_from(diff);
        }
    }
    pub fn first(&mut self) {
        self.list_state.select(Some(0));
    }
    pub fn last(&mut self) {
        self.list_state
            .select(Some(self.difficulties.len()));
    }
    pub fn prev(&mut self) {
        let len = self.difficulties.len().max(1);
        let i = self
            .list_state
            .selected()
            .map_or(0, |i| (len + i - 1) % len);
        self.list_state.select(Some(i));
    }
    pub fn next(&mut self) {
        let len = self.difficulties.len().max(1);
        let i = self
            .list_state
            .selected()
            .map_or(0, |i| (len + i + 1) % len);
        self.list_state.select(Some(i));
    }
}
