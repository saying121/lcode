pub mod tab0;
pub mod tab1;
pub mod tab2;
pub mod tab3;

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
    if let Event::Key(keyevent) = event {
        let KeyEvent {
            code, modifiers, ..
        } = keyevent;
        match code {
            KeyCode::Char('l') if *modifiers == KeyModifiers::CONTROL => {
                redraw(terminal, app)?
            }
            KeyCode::Tab | KeyCode::Right => app.next_tab()?,
            KeyCode::BackTab | KeyCode::Left => app.prev_tab()?,
            _ => {}
        }
    }
    Ok(())
}
