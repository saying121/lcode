use std::io::Stdout;

use super::common_keymap;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
};
use miette::{IntoDiagnostic, Result};
use ratatui::prelude::Backend;
use ratatui::Terminal;
use tui_textarea::{Input, Key};

use crate::mytui::{
    app::{App, InputMode},
    myevent::UserEvent,
    redraw,
};

pub async fn tab0_keymap<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    match app.input_line_mode {
        InputMode::Normal => match event {
            Event::Key(keyevent) => match keyevent.code {
                KeyCode::Char('e') => {
                    app.input_line_mode = InputMode::Insert;
                }
                KeyCode::Char('S') => {
                    app.sync_state = true;
                    app.tx
                        .send(UserEvent::StartSync)
                        .into_diagnostic()?;
                }
                KeyCode::Char('g') => {
                    if let Event::Key(key) = event::read().into_diagnostic()? {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('g') => app.first_item(),
                                _ => {}
                            }
                        }
                    }
                }
                KeyCode::Char('G') => app.last_item(),
                KeyCode::Enter => app.goto_tab(1)?,
                KeyCode::Down | KeyCode::Char('j') => app.next_item(),
                KeyCode::Up | KeyCode::Char('k') => app.previous_item(),
                KeyCode::Char('r') if keyevent.modifiers == KeyModifiers::CONTROL => {
                    app.tx
                        .send(UserEvent::GetQs((app.current_qs(), true)))
                        .into_diagnostic()?;
                }
                KeyCode::Char('o') => {
                    // stop listen keyevent
                    *app.editor_flag.lock().unwrap() = false;
                    app.confirm().await?;
                    // start listen keyevent
                    *app.editor_flag.lock().unwrap() = true;
                    app.editor_cond.notify_one();

                    use crossterm::terminal::EnterAlternateScreen;
                    execute!(stdout, EnterAlternateScreen).into_diagnostic()?;

                    redraw(terminal, app)?;
                }
                _ => {
                    common_keymap(app, terminal, event, stdout).await?;
                }
            },
            _ => {}
        },
        InputMode::Insert => match event.clone().into() {
            Input { key: Key::Esc, .. } => app.input_line_mode = InputMode::Normal,
            Input {
                key: Key::Char('m'),
                ctrl: true,
                ..
            }
            | Input {
                key: Key::Enter, ..
            } => {}
            input => {
                app.text_line.input(input);
            }
        },
    };
    Ok(())
}
