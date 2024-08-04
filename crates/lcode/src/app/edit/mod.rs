mod cmds;

use crossterm::event::Event as CrossEvent;
use tui_textarea::{Input, Key};

use self::cmds::{button, codeblock, content, submit, test};

// tab1 edit
#[derive(Debug)]
#[derive(Default)]
pub struct EditCode<'tab1> {
    pub code_block: codeblock::CodeBlock<'tab1>,
    pub qs_content: content::ContentState,
    pub button: button::ButState,
    pub submit: submit::SubmitState,
    pub test: test::TestState,
}

impl<'tab1> EditCode<'tab1> {
    pub fn normal_map(&mut self, event: CrossEvent) -> bool {
        self.code_block.normal_map(event)
    }

    pub fn insert_keymap(&mut self, event: CrossEvent) -> bool {
        match event.into() {
            Input { key: Key::Esc, .. } => self.be_code_normal(),
            input => self.code_block.insert_map(input), /* Use default key mappings in insert mode(emacs) */
        }
    }

    pub fn be_code_normal(&mut self) -> bool {
        self.code_block.be_code_normal()
    }
    pub fn start_edit_tui(&mut self) -> bool {
        self.code_block.start_edit_tui()
    }

    /// when true, mean can add a new test case
    pub const fn add_test_case(&self) -> bool {
        self.submit.need_add()
    }
}

// Show only one pop view every time.
impl<'tab1> EditCode<'tab1> {
    pub fn toggle_menu(&mut self) -> bool {
        self.button.toggle();
        self.test.close();
        self.submit.close();
        true
    }
    pub fn toggle_test_res(&mut self) -> bool {
        self.test.toggle();
        self.button.close();
        self.submit.close();
        true
    }
    pub fn toggle_submit_res(&mut self) -> bool {
        self.submit.toggle();
        self.test.close();
        self.button.close();
        true
    }
}

impl<'tab1> EditCode<'tab1> {
    pub fn close_pop(&mut self) -> bool {
        if self.test.show {
            self.test.close();
        }
        else if self.submit.show {
            self.submit.close();
        }
        else if self.button.show {
            self.button.close();
        }
        true
    }

    pub fn vertical_scroll_j(&mut self) -> bool {
        if self.test.show {
            self.test.down();
        }
        else if self.submit.show {
            self.submit.down();
        }
        else if !self.button.show {
            self.qs_content.down();
        }
        true
    }

    pub fn vertical_scroll_k(&mut self) -> bool {
        if self.test.show {
            self.test.up();
        }
        else if self.submit.show {
            self.submit.up();
        }
        else if !self.button.show {
            self.qs_content.up();
        }
        true
    }

    pub fn horizontal_scroll_h(&mut self) -> bool {
        if self.test.show {
            self.test.left();
        }
        else if self.submit.show {
            self.submit.left();
        }
        else if self.button.show {
            self.button.left();
        }
        else {
            self.qs_content.left();
        }
        true
    }

    pub fn horizontal_scroll_l(&mut self) -> bool {
        if self.test.show {
            self.test.right();
        }
        else if self.submit.show {
            self.submit.right();
        }
        else if self.button.show {
            self.button.right();
        }
        else {
            self.qs_content.right();
        }
        true
    }

    pub fn vertical_scroll_gg(&mut self) -> bool {
        if self.submit.show {
            self.submit.first();
        }
        else if self.test.show {
            self.test.first();
        }
        else {
            self.qs_content.top();
        }
        true
    }

    #[allow(non_snake_case)]
    pub fn vertical_scroll_G(&mut self) -> bool {
        if self.submit.show {
            self.submit.last();
        }
        else if self.test.show {
            self.test.last();
        }
        else {
            self.qs_content.bottom();
        }
        true
    }
    /// goto first column
    pub fn goto_pop_head(&mut self) -> bool {
        if self.submit.show {
            self.submit.goto_head();
        }
        if self.test.show {
            self.test.goto_head();
        }
        true
    }
}
