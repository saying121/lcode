use crossterm::event::{self, Event as CrossEvent, KeyCode};
use tui_textarea::{CursorMove, Input, Key, Scrolling, TextArea};

use crate::app::TuiMode;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct CodeBlock<'block> {
    pub code_block: TextArea<'block>,
    pub mode: TuiMode,
}

impl<'block> CodeBlock<'block> {
    pub fn quit_edit_tui(&mut self) -> bool {
        self.mode = TuiMode::OutEdit;
        true
    }
    pub fn be_code_insert(&mut self) -> bool {
        self.mode = TuiMode::Insert;
        true
    }
    pub fn be_code_normal(&mut self) -> bool {
        self.mode = TuiMode::Normal;
        true
    }
    pub fn start_edit_tui(&mut self) -> bool {
        self.mode = TuiMode::Normal;
        true
    }
    pub fn insert_map(&mut self, input: Input) -> bool {
        self.code_block.input(input)
    }
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
}
