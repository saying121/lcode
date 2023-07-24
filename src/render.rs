use miette::{Error, IntoDiagnostic};
use pulldown_cmark::{Options, Parser};
use pulldown_cmark_mdcat::{
    push_tty, resources::FileResourceHandler, Environment, Settings,
    TerminalProgram, TerminalSize, Theme,
};
use std::io::stdout;
use syntect::parsing::SyntaxSet;

/// 渲染markdown文本
///
/// * `text`: markdown 的字符串
pub fn render_md_str(text: &str) -> Result<(), Error> {
    let terminal = TerminalProgram::detect();

    let set = Settings {
        terminal_capabilities: terminal.capabilities(),
        terminal_size: TerminalSize::detect().unwrap_or_default(),
        syntax_set: &SyntaxSet::load_defaults_newlines(),
        theme: Theme::default(),
    };
    let home = dirs::home_dir().unwrap();

    let env = Environment::for_local_directory(&home).into_diagnostic()?;

    let hd = FileResourceHandler::new(100);

    let parser = Parser::new_ext(
        &text,
        Options::all()
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_STRIKETHROUGH,
    );

    // let mut out = std::io::Cursor::new(vec![]);
    // push_tty(&set, &env, &hd, &mut out, parser).unwrap();

    // 渲染到终端中
    push_tty(&set, &env, &hd, &mut stdout(), parser).unwrap();

    Ok(())
}

pub fn from_html_to_md(html: &str) -> String {
    use html2text::from_read;
    from_read(html.as_bytes(), 80)
}
