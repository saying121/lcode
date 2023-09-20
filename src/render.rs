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
use ratatui::text::Line;
use regex::{Captures, Regex};
use syntect::parsing::SyntaxSet;

use crate::{config::global::glob_user_config, leetcode::qs_detail::Question};

pub enum StTy {
    Str,
    Tty,
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

    rendering(set, md_str, StTy::Tty)?;

    Ok(())
}

pub trait Render {
    /// for ratatui paragraph
    fn to_tui_mdvec(&self, _width: usize) -> Vec<String> {
        vec![]
    }
    /// for ratatui paragraph
    fn to_tui_vec(&self) -> Vec<Line>;
    /// Get a Rendered question String
    fn to_rendered_str(&self, _col: u16, _row: u16) -> Result<String> {
        Ok(String::new())
    }
    /// md str but not render
    fn to_md_str(&self) -> String {
        String::new()
    }
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

    let res = rendering(set, md_str, StTy::Str)?;

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

    rendering(set, md_str, StTy::Tty)?;

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
        StTy::Str => {
            let mut out = std::io::Cursor::new(vec![]);
            push_tty(&set, &env, &handle, &mut out, parser).unwrap();
            out.rewind().into_diagnostic()?;

            let mut temp = String::new();
            out.read_to_string(&mut temp)
                .into_diagnostic()?;
            temp
        }
        StTy::Tty => {
            // rendr to terminal
            push_tty(&set, &env, &handle, &mut stdout(), parser).unwrap();
            String::new()
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
        .replace("\\n", "\n");

    let md_str = html2text::from_read(content.as_bytes(), 80);

    let md_str = format!("{}\n---\n\n{}\n---", qs, md_str);

    md_str
}

pub fn gen_sub_sup_script(content: &str) -> String {
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

fn superscript(n: u32) -> String {
    match n {
        0 => "⁰".to_owned(),
        1 => "¹".to_owned(),
        2 => "²".to_owned(),
        3 => "³".to_owned(),
        4 => "⁴".to_owned(),
        5 => "⁵".to_owned(),
        6 => "⁶".to_owned(),
        7 => "⁷".to_owned(),
        8 => "⁸".to_owned(),
        9 => "⁹".to_owned(),
        mut num => {
            let mut res = String::new();
            while num > 0 {
                res = superscript(num % 10).to_owned() + &res;
                num /= 10;
            }
            res
        }
    }
}

fn subscript(n: u32) -> String {
    match n {
        0 => "₀".to_owned(),
        1 => "₁".to_owned(),
        2 => "₂".to_owned(),
        3 => "₃".to_owned(),
        4 => "₄".to_owned(),
        5 => "₅".to_owned(),
        6 => "₆".to_owned(),
        7 => "₇".to_owned(),
        8 => "₈".to_owned(),
        9 => "₉".to_owned(),
        mut num => {
            let mut res = String::new();
            while num > 0 {
                res = subscript(num % 10).to_owned() + &res;
                num /= 10;
            }
            res
        }
    }
}
