use std::io::Stdout;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
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
    match app.filter_index {
        0 => all_topic(app, terminal, event, stdout).await?,
        1 => user_topic(app, terminal, event, stdout).await?,
        2 => filtered_qs(app, terminal, event, stdout).await?,
        _ => common_keymap(app, terminal, event, stdout).await?,
    }
    Ok(())
}

async fn filtered_qs<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    match event {
        Event::Key(keyevent) => match keyevent.code {
            KeyCode::Char('j') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.filter_index = 1;
            }
            KeyCode::Char('k') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.filter_index = 0;
            }
            KeyCode::Char('h') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.filter_index = 0;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                app.next_topic_qs();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.prev_topic_qs();
            }
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('g') => app.first_topic_qs(),
                            _ => {}
                        }
                    }
                }
            }
            KeyCode::Char('G') => app.last_topic_qs(),
            KeyCode::Enter => app.confirm_filtered_qs(),
            _ => common_keymap(app, terminal, event, stdout).await?,
        },
        _ => {
            common_keymap(app, terminal, event, stdout).await?;
        }
    }

    Ok(())
}

async fn user_topic<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    match event {
        Event::Key(keyevent) => match keyevent.code {
            KeyCode::Char('j') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.filter_index = 1;
            }
            KeyCode::Char('k') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.filter_index = 0;
            }
            KeyCode::Char('l') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.filter_index = 2;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                app.next_user_topic();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.prev_user_topic();
            }
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('g') => {
                                app.first_topic();
                            }
                            _ => {}
                        }
                    }
                }
            }
            KeyCode::Char('G') => {
                app.last_topic();
            }
            KeyCode::Enter => {
                app.add_or_rm_user_topic().await;
            }
            _ => common_keymap(app, terminal, event, stdout).await?,
        },
        _ => {
            common_keymap(app, terminal, event, stdout).await?;
        }
    }

    Ok(())
}
async fn all_topic<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    match event {
        Event::Key(keyevent) => match keyevent.code {
            KeyCode::Char('j') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.filter_index = 1;
            }
            KeyCode::Char('k') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.filter_index = 0;
            }
            KeyCode::Char('l') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.filter_index = 2;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                app.next_topic();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.prev_topic();
            }
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('g') => {
                                app.first_topic();
                            }
                            _ => {}
                        }
                    }
                }
            }
            KeyCode::Char('G') => {
                app.last_topic();
            }
            KeyCode::Enter => {
                app.add_or_rm_user_topic().await;
            }
            _ => common_keymap(app, terminal, event, stdout).await?,
        },
        _ => {
            common_keymap(app, terminal, event, stdout).await?;
        }
    }

    Ok(())
}
