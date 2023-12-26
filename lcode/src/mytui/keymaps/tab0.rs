use std::io::Stdout;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute
};
use miette::{IntoDiagnostic, Result};
use tui_textarea::{Input, Key};

use super::common_keymap;
use crate::{
    editor::edit_config,
    leetcode::IdSlug,
    mytui::{
        app::{App, TuiMode},
        term::Term
    }
};

pub async fn init(
    app: &mut App<'_>,
    terminal: &mut Term,
    event: &Event,
    stdout: &mut Stdout
) -> Result<()> {
    match app.tab0.input_line_mode {
        TuiMode::Normal => {
            if let Event::Key(keyevent) = event {
                match keyevent.code {
                    KeyCode::Char('C') => {
                        edit_config().await?;

                        terminal.redraw()?;
                    },
                    KeyCode::Char('o') => {
                        app.tab0.edit_cur_qs().await?;

                        app.get_code(&app.cur_qs.clone())
                            .await?;

                        terminal.redraw()?;
                    },
                    _ => common_keymap(app, terminal, event, stdout).await?
                }
            }
        },
        TuiMode::Insert => match event.clone().into() {
            Input {
                key: Key::Esc, ..
            } => app.tab0.be_normal(),
            input => {
                app.tab0.text_line.input(input);
            }
        }
    };
    Ok(())
}
