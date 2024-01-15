pub mod run_res;
pub mod qs_detail;

use std::{
    env,
    io::{stdout, Read, Seek},
};

use pulldown_cmark::{Options, Parser};
use pulldown_cmark_mdcat::{
    push_tty, resources::FileResourceHandler, Environment, Settings, TerminalProgram, TerminalSize,
    Theme,
};
use ratatui::text::Line;
use regex::{Captures, Regex};
use syntect::parsing::SyntaxSet;

#[derive(Copy, Clone)]
pub enum SupSub {
    Sup,
    Sub,
}

#[derive(Copy, Clone)]
pub enum StTy {
    Str,
    Tty,
}

pub trait Render {
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
            terminal_size:         TerminalSize::detect().unwrap_or_default(),
            syntax_set:            &SyntaxSet::load_defaults_newlines(),
            theme:                 Theme::default(),
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
            terminal_size:         term_size,
            syntax_set:            &SyntaxSet::load_defaults_newlines(),
            theme:                 Theme::default(),
        };

        rendering(&set, &self.to_md_str(with_env), StTy::Str).expect("rendering error")
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
        },
        StTy::Tty => {
            // rendr to terminal
            push_tty(set, &env, &handle, &mut stdout(), parser).unwrap();
            None
        },
    }
}

pub fn to_sub_sup_script(content: &str) -> String {
    let sup_re = Regex::new("<sup>(?P<num>[0-9]*)</sup>").unwrap();
    let sub_re = Regex::new("<sub>(?P<num>[0-9]*)</sub>").unwrap();

    let content = sup_re.replace_all(content, |cap: &Captures| {
        let num = cap["num"]
            .parse()
            .unwrap_or_default();
        superscript(num, SupSub::Sup)
    });

    let content = sub_re.replace_all(&content, |cap: &Captures| {
        let num = cap["num"]
            .parse()
            .unwrap_or_default();
        superscript(num, SupSub::Sub)
    });

    content.to_string()
}

const SUPER_NUM: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
const SUB_NUM: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

pub fn superscript(n: usize, sub_or_sup: SupSub) -> String {
    let sub_or_sup = match sub_or_sup {
        SupSub::Sup => SUPER_NUM,
        SupSub::Sub => SUB_NUM,
    };
    match n {
        0..=9 => sub_or_sup[n].to_string(),
        mut num => {
            // 2 is enough, avoid frequently create string
            let mut res = String::with_capacity(2);
            while num > 0 {
                res.push(sub_or_sup[num % 10]);
                num /= 10;
            }
            res.chars()
                .rev()
                .collect::<String>()
        },
    }
}
