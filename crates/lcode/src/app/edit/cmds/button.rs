use crate::mytui::my_widget::botton::{ButtonState, ButtonStates};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct ButState {
    pub button_state:  ButtonStates,
    pub select_button: usize,
    pub show:          bool,
    pub submitting:    bool,
}

impl ButState {
    pub fn active_but(&mut self) {
        self.button_state.states[self.select_button] = ButtonState::Active;
    }
    pub fn done(&mut self) {
        self.submitting = false;
    }
    pub fn start(&mut self) {
        self.submitting = false;
    }
    pub fn test_done(&mut self) {
        self.done();
        self.button_state.states[0] = ButtonState::Normal;
    }
    pub fn submit_done(&mut self) {
        self.done();
        self.button_state.states[1] = ButtonState::Normal;
    }
}

impl ButState {
    #[allow(dead_code)]
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
