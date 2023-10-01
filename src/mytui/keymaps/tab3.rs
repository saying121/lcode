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
                app.tab3.next_keymap();
            }
            KeyCode::Char('k') => {
                app.tab3.prev_keymap();
            }
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('g') {
                        app.tab3.first_keymap();
                    }
                }
            }
            KeyCode::Char('G') => {
                app.tab3.last_keymap();
            }
            KeyCode::Enter | KeyCode::Char('o' | 'O')
                if 0 == app
                    .tab3
                    .keymaps_state
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
