use crate::{config::global::global_user_config, leetcode::question_detail::Question};
use miette::{IntoDiagnostic, Result};
use pulldown_cmark::{Options, Parser};
use pulldown_cmark_mdcat::{
    push_tty, resources::FileResourceHandler, Environment, Settings, TerminalProgram,
    TerminalSize, Theme,
};
use std::{
    env,
    io::{stdout, Read, Seek},
};
use syntect::parsing::SyntaxSet;

enum StTy {
    STR,
    TTY,
}

/// Get a Rendered question String
pub fn get_qs_rendered_str(qs: &Question, col: u16, row: u16) -> Result<String> {
    let md_str = pre_render(&qs)?;

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

/// Render a question to terminal.
pub fn render_qs_to_tty(qs: Question) -> Result<()> {
    let md_str = pre_render(&qs)?;

    let set = Settings {
        terminal_capabilities: TerminalProgram::detect().capabilities(),
        terminal_size: TerminalSize::detect().unwrap_or_default(),
        syntax_set: &SyntaxSet::load_defaults_newlines(),
        theme: Theme::default(),
    };

    rendering(set, md_str, StTy::TTY)?;

    Ok(())
}

/// Get arendered markdown String
pub fn get_rendered_str(md_str: String,col: u16,row: u16) -> Result<String> {
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

fn rendering(set: Settings, md_str: String, target: StTy) -> Result<String> {
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

fn from_html_to_md(html: &str) -> String {
    use html2text::from_read;
    from_read(html.as_bytes(), 80)
}

/// uniform treatment Question detail to String
fn pre_render(qs: &Question) -> Result<String> {
    let user = global_user_config();
    let content = match user.tongue.as_str() {
        "cn" => qs
            .translated_content
            .as_ref()
            .map_or("Not Exists".to_string(), |v| v.clone())
            .as_str()
            .trim_matches('"')
            .replace("\\n", "")
            .to_owned(),
        "en" => qs.content.to_owned().unwrap(),
        _ => qs.content.to_owned().unwrap(),
    };

    let md_str = from_html_to_md(&content);

    let md_str = format!("{}\n\n---\n{}\n---", qs, md_str);

    Ok(md_str)
}
