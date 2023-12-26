pub mod tab0;
pub mod tab1;
pub mod tab2;
pub mod tab3;

use std::io::Stdout;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use miette::Result;

use super::{app::App, term::Term};

#[allow(renamed_and_removed_lints)]
#[allow(unused_async)]
pub(super) async fn common_keymap(
    app: &mut App<'_>,
    terminal: &mut Term,
    event: &Event,
    _stdout: &mut Stdout,
) -> Result<()> {
    if let Event::Key(keyevent) = event {
        let KeyEvent {
            code, modifiers, ..
        } = keyevent;
        match code {
            KeyCode::Char('l') if *modifiers == KeyModifiers::CONTROL => {
                terminal.redraw()?;
            }
            KeyCode::Tab | KeyCode::Right => app.next_tab(),
            KeyCode::BackTab | KeyCode::Left => app.prev_tab(),
            _ => {}
        }
    }
    Ok(())
}
