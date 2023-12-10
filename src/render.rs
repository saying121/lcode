use std::{
    env,
    io::{stdout, Read, Seek},
};

use pulldown_cmark::{Options, Parser};
use pulldown_cmark_mdcat::{
    push_tty, resources::FileResourceHandler, Environment, Settings, TerminalProgram,
    TerminalSize, Theme,
};
use ratatui::text::Line;
use regex::{Captures, Regex};
use syntect::parsing::SyntaxSet;

#[derive(Copy, Clone)]
pub enum StTy {
    Str,
    Tty,
}

pub trait Render {
    /// for ratatui paragraph
    fn to_tui_mdvec(&self, _width: usize) -> Vec<String> {
        vec![]
    }

    /// uniform treatment `Question` detail to markdown String
    ///
    /// * `_with_env`: whether display Compile Environment
    fn to_md_str(&self, _with_env: bool) -> String {
        String::new()
    }

    /// for ratatui paragraph
    fn to_tui_vec(&self) -> Vec<Line>;
    /// render to terminal
    fn render_to_terminal(&self) {
        let set = Settings {
            terminal_capabilities: TerminalProgram::detect().capabilities(),
            terminal_size: TerminalSize::detect().unwrap_or_default(),
            syntax_set: &SyntaxSet::load_defaults_newlines(),
            theme: Theme::default(),
        };

        rendering(&set, &self.to_md_str(false), StTy::Tty);
    }

    /// Get a rendered markdown String
    ///
    /// * `with_env`: whether display Compile Environment
    /// * `col`: width
    /// * `row`: height
    fn to_rendered_str(&self, with_env: bool, col: u16, row: u16) -> String {
        let term_size = TerminalSize {
            columns: col,
            rows: row,
            ..Default::default()
        };
        let set = Settings {
            terminal_capabilities: TerminalProgram::detect().capabilities(),
            terminal_size: term_size,
            syntax_set: &SyntaxSet::load_defaults_newlines(),
            theme: Theme::default(),
        };

        rendering(&set, &self.to_md_str(with_env), StTy::Str).unwrap()
    }
}

/// uniform render
///
/// * `set`: `Settings`
/// * `md_str`: String
/// * `target`: to terminal(return `None`) or rendered string(return `Some(String)`)
pub fn rendering(set: &Settings, md_str: &str, target: StTy) -> Option<String> {
    let pwd = env::current_dir().ok()?;
    let env = Environment::for_local_directory(&pwd).ok()?;
    let handle = FileResourceHandler::new(104_857_600);

    let parser = Parser::new_ext(md_str, Options::all());

    match target {
        StTy::Str => {
            // rendr to `out`
            let mut out = std::io::Cursor::new(vec![]);
            push_tty(set, &env, &handle, &mut out, parser).unwrap();
            out.rewind().ok()?;

            let mut temp = String::new();
            out.read_to_string(&mut temp)
                .ok()?;
            Some(temp)
        }
        StTy::Tty => {
            // rendr to terminal
            push_tty(set, &env, &handle, &mut stdout(), parser).unwrap();
            None
        }
    }
}

pub fn to_sub_sup_script(content: &str) -> String {
    let sup_re = Regex::new("<sup>(?P<num>[0-9]*)</sup>").unwrap();
    let sub_re = Regex::new("<sub>(?P<num>[0-9]*)</sub>").unwrap();

    let content = sup_re.replace_all(content, |cap: &Captures| {
        let num = cap["num"]
            .to_string()
            .parse()
            .unwrap();
        superscript(num)
    });

    let content = sub_re.replace_all(&content, |cap: &Captures| {
        let num = cap["num"]
            .to_string()
            .parse()
            .unwrap();
        subscript(num)
    });

    content.to_string()
}

const SUPER_NUM: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
pub fn superscript(n: usize) -> String {
    match n {
        0..=9 => SUPER_NUM[n].to_string(),
        mut num => {
            let mut res = String::new();
            while num > 0 {
                res = format!("{}{}", SUPER_NUM[num % 10], res);
                num /= 10;
            }
            res
        }
    }
}

const SUB_NUM: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];
pub fn subscript(n: usize) -> String {
    match n {
        0..=9 => SUB_NUM[n].to_string(),
        mut num => {
            let mut res = String::new();
            while num > 0 {
                res = format!("{}{}", SUB_NUM[num % 10], res);
                num /= 10;
            }
            res
        }
    }
}
