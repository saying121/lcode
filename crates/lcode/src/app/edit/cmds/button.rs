use crate::mytui::my_widget::botton::{ButtonState, ButtonStates};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct ButState {
    pub show: bool,
    pub state: ButtonStates,
    pub selected: usize,
    pub submitting: bool,
}

impl ButState {
    pub fn active_cur_but(&mut self) {
        self.state.states[self.selected] = ButtonState::Active;
    }
    pub fn done(&mut self) {
        self.submitting = false;
    }
    pub fn start(&mut self) {
        self.submitting = false;
    }
    pub fn test_done(&mut self) {
        self.done();
        self.state.states[0] = ButtonState::Normal;
    }
    pub fn submit_done(&mut self) {
        self.done();
        self.state.states[1] = ButtonState::Normal;
    }
}

impl ButState {
    #[expect(dead_code, reason = "util fn")]
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
        if self.state.states[self.selected] != ButtonState::Active {
            self.state.states[self.selected] = ButtonState::Normal;
        }
        self.selected = self.selected.saturating_sub(1);
        if self.state.states[self.selected] != ButtonState::Active {
            self.state.states[self.selected] = ButtonState::Selected;
        }
    }
    pub fn right(&mut self) {
        if self.state.states[self.selected] != ButtonState::Active {
            self.state.states[self.selected] = ButtonState::Normal;
        }

        self.selected = self.selected.saturating_add(1).min(1);

        if self.state.states[self.selected] != ButtonState::Active {
            self.state.states[self.selected] = ButtonState::Selected;
        }
    }
}
