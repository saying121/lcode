pub mod tab0;
pub mod tab1;

use std::io::Stdout;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use miette::Result;
use ratatui::{prelude::Backend, Terminal};

use super::{app::App, redraw};

pub(super) async fn common_keymap<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    _stdout: &mut Stdout,
) -> Result<()> {
    match event {
        Event::Key(keyevent) => match keyevent {
            KeyEvent {
                code, modifiers, ..
            } => match code {
                KeyCode::Char('r') if *modifiers == KeyModifiers::CONTROL => {
                    redraw(terminal, app)?
                }
                KeyCode::Tab | KeyCode::Right => app.next_tab()?,
                KeyCode::BackTab | KeyCode::Left => app.prev_tab()?,

                _ => {}
            },
        },
        _ => {}
    }
    Ok(())
}
