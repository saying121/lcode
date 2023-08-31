use std::{
    env,
    io::{stdout, Read, Seek},
};

use miette::{IntoDiagnostic, Result};
use pulldown_cmark::{Options, Parser};
use pulldown_cmark_mdcat::{
    push_tty, resources::FileResourceHandler, Environment, Settings, TerminalProgram,
    TerminalSize, Theme,
};
use regex::{Captures, Regex};
use syntect::parsing::SyntaxSet;

use crate::{config::global::glob_user_config, leetcode::qs_detail::Question};

pub enum StTy {
    STR,
    TTY,
}

/// Render a question to terminal.
pub fn render_qs_to_tty(qs: Question) -> Result<()> {
    let md_str = pre_render(&qs);

    let set = Settings {
        terminal_capabilities: TerminalProgram::detect().capabilities(),
        terminal_size: TerminalSize::detect().unwrap_or_default(),
        syntax_set: &SyntaxSet::load_defaults_newlines(),
        theme: Theme::default(),
    };

    rendering(set, md_str, StTy::TTY)?;

    Ok(())
}

pub trait Render {
    /// for ratatui paragraph
    fn to_tui_mdvec(&self, width: usize) -> Vec<String>;
    /// for ratatui paragraph
    fn to_tui_vec(&self) -> Vec<String>;
    /// Get a Rendered question String
    fn to_rendered_str(&self, col: u16, row: u16) -> Result<String>;
    fn to_md_str(&self) -> String;
}

/// Get arendered markdown String
pub fn get_rendered_str(md_str: String, col: u16, row: u16) -> Result<String> {
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

    let res = rendering(set, md_str, StTy::STR)?;

    Ok(res)
}

/// render a markdown String to terminal
pub fn render_str(md_str: String) -> Result<()> {
    let set = Settings {
        terminal_capabilities: TerminalProgram::detect().capabilities(),
        terminal_size: TerminalSize::detect().unwrap_or_default(),
        syntax_set: &SyntaxSet::load_defaults_newlines(),
        theme: Theme::default(),
    };

    rendering(set, md_str, StTy::TTY)?;

    Ok(())
}

pub fn rendering(set: Settings, md_str: String, target: StTy) -> Result<String> {
    let pwd = env::current_dir().into_diagnostic()?;
    let env = Environment::for_local_directory(&pwd).into_diagnostic()?;
    let handle = FileResourceHandler::new(100);

    let parser = Parser::new_ext(
        &md_str,
        Options::all() | Options::ENABLE_TASKLISTS | Options::ENABLE_STRIKETHROUGH,
    );

    let res = match target {
        StTy::STR => {
            let mut out = std::io::Cursor::new(vec![]);
            push_tty(&set, &env, &handle, &mut out, parser).unwrap();
            out.rewind().into_diagnostic()?;

            let mut temp = "".to_string();
            out.read_to_string(&mut temp)
                .into_diagnostic()?;
            temp
        }
        StTy::TTY => {
            // rendr to terminal
            push_tty(&set, &env, &handle, &mut stdout(), parser).unwrap();
            "".to_string()
        }
    };

    Ok(res)
}

/// uniform treatment Question detail to String
pub fn pre_render(qs: &Question) -> String {
    let content = match glob_user_config().translate {
        true => qs
            .translated_content
            .clone()
            .unwrap_or_default(),
        false => qs
            .content
            .clone()
            .unwrap_or_default(),
    };

    let content = gen_sub_sup_script(&content)
        .trim_matches('"')
        .replace("\\n", "");

    let md_str = html2text::from_read(content.as_bytes(), 80);

    let md_str = format!("{}\n\n---\n{}\n---", qs, md_str);

    md_str
}

pub fn gen_sub_sup_script(content: &str) -> String {
    let sup_re = Regex::new(r"<sup>(?P<num>[0-9]*)</sup>").unwrap();
    let sub_re = Regex::new(r"<sub>(?P<num>[0-9]*)</sub>").unwrap();

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

fn superscript(n: u32) -> String {
    match n {
        0 => "⁰".to_string(),
        1 => "¹".to_string(),
        2 => "²".to_string(),
        3 => "³".to_string(),
        4 => "⁴".to_string(),
        5 => "⁵".to_string(),
        6 => "⁶".to_string(),
        7 => "⁷".to_string(),
        8 => "⁸".to_string(),
        9 => "⁹".to_string(),
        mut num => {
            let mut res = "".to_string();
            while num > 0 {
                res = format!("{}", superscript(num % 10)) + &res;
                num /= 10;
            }
            res
        }
    }
}

fn subscript(n: u32) -> String {
    match n {
        0 => "₀".to_string(),
        1 => "₁".to_string(),
        2 => "₂".to_string(),
        3 => "₃".to_string(),
        4 => "₄".to_string(),
        5 => "₅".to_string(),
        6 => "₆".to_string(),
        7 => "₇".to_string(),
        8 => "₈".to_string(),
        9 => "₉".to_string(),
        mut num => {
            let mut res = "".to_string();
            while num > 0 {
                res = format!("{}", subscript(num % 10)) + &res;
                num /= 10;
            }
            res
        }
    }
}
