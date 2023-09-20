use std::io::Stdout;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use miette::{IntoDiagnostic, Result};
use ratatui::{prelude::Backend, Terminal};

use super::common_keymap;
use crate::mytui::app::App;

pub async fn init<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    match event {
        Event::Key(keyevent) => match keyevent.code {
            KeyCode::Char('j') => {
                app.next_list();
            }
            KeyCode::Char('k') => {
                app.prev_list();
            }
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press {
                        if let KeyCode::Char('g') = key.code {
                            app.first_list();
                        }
                    }
                }
            }
            KeyCode::Char('G') => {
                app.last_list();
            }
            KeyCode::Enter | KeyCode::Char('o' | 'O')
                if 0 == app
                    .l_state
                    .selected()
                    .unwrap_or_default() =>
            {
                crate::star();
            }
            _ => {
                common_keymap(app, terminal, event, stdout).await?;
            }
        },
        _ => {
            common_keymap(app, terminal, event, stdout).await?;
        }
    }
    Ok(())
}
