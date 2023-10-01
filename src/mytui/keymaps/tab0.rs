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

use crate::{
    editor::edit_config,
    leetcode::IdSlug,
    mytui::{
        app::{App, InputMode},
        myevent::UserEvent,
        redraw,
    },
};

pub async fn tab0_keymap<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    match app.tab0.input_line_mode {
        InputMode::Normal => {
            if let Event::Key(keyevent) = event {
                match keyevent.code {
                    KeyCode::Char('C') => {
                        // stop listen keyevent
                        *app.editor_flag.lock().unwrap() = false;
                        edit_config().await?;
                        // start listen keyevent
                        *app.editor_flag.lock().unwrap() = true;
                        app.editor_cond.notify_one();

                        // let res = USER_CONFIG
                        //     .set(read_config::get_user_conf().unwrap_or_default());

                        use crossterm::terminal::EnterAlternateScreen;
                        execute!(stdout, EnterAlternateScreen).into_diagnostic()?;

                        redraw(terminal, app)?;
                    }
                    KeyCode::Char('e') => {
                        app.tab0.input_line_mode = InputMode::Insert;
                    }
                    KeyCode::Char('S') => {
                        app.sync_state = true;
                        app.tx
                            .send(UserEvent::StartSync(false))
                            .into_diagnostic()?;
                    }
                    KeyCode::Char('g') => {
                        if let Event::Key(key) = event::read().into_diagnostic()? {
                            if key.kind == KeyEventKind::Press {
                                if let KeyCode::Char('g') = key.code {
                                    app.tab0.first_question()
                                }
                            }
                        }
                    }
                    KeyCode::Char('G') => app.tab0.last_question(),
                    KeyCode::Enter => app.goto_tab(1)?,
                    KeyCode::Down | KeyCode::Char('j') => app.tab0.next_question(),
                    KeyCode::Up | KeyCode::Char('k') => app.tab0.previous_question(),
                    KeyCode::Char('r') if keyevent.modifiers == KeyModifiers::CONTROL => {
                        app.tx
                            .send(UserEvent::GetQs((
                                IdSlug::Id(app.tab0.current_qs()),
                                true,
                            )))
                            .into_diagnostic()?;
                    }
                    KeyCode::Char('o') => {
                        // stop listen keyevent
                        *app.editor_flag.lock().unwrap() = false;
                        app.tab0.confirm_qs().await?;
                        // start listen keyevent
                        *app.editor_flag.lock().unwrap() = true;
                        app.editor_cond.notify_one();
                        app.get_code(&app.tab0.cur_qs.clone())
                            .await?;

                        use crossterm::terminal::EnterAlternateScreen;
                        execute!(stdout, EnterAlternateScreen).into_diagnostic()?;

                        redraw(terminal, app)?;
                    }
                    _ => {
                        common_keymap(app, terminal, event, stdout).await?;
                    }
                }
            }
        }
        InputMode::Insert => match event.clone().into() {
            Input { key: Key::Esc, .. } => app.tab0.input_line_mode = InputMode::Normal,
            Input {
                key: Key::Char('m'),
                ctrl: true,
                ..
            }
            | Input {
                key: Key::Enter, ..
            } => {}
            input => {
                app.tab0.text_line.input(input);
            }
        },
    };
    Ok(())
}
