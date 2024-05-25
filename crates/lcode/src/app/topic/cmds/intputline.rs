use tui_textarea::{Input, TextArea};

use crate::app::TuiMode;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct InputLine<'line> {
    pub mode:      TuiMode,
    pub text_line: TextArea<'line>,
}

impl<'line> InputLine<'line> {
    /// return if the input modified text contents or not
    pub fn handle_input(&mut self, input: Input) -> bool {
        self.text_line.input(input)
    }
    pub fn out_edit(&mut self) {
        self.mode = TuiMode::OutEdit;
    }
    pub fn enter_input(&mut self) {
        self.mode = TuiMode::Insert;
    }
    pub fn first_line(&self) -> &str {
        &self.text_line.lines()[0]
    }
}
