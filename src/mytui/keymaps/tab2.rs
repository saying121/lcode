use std::io::Stdout;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
};
use miette::{IntoDiagnostic, Result};
use ratatui::{prelude::Backend, Terminal};

use super::common_keymap;
use crate::mytui::{app::App, myevent::UserEvent, redraw};

pub async fn init<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    match app.tab2.filter_index {
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
                app.tab2.filter_index = 1;
            }
            KeyCode::Char('k') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.filter_index = 0;
            }
            KeyCode::Char('h') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.filter_index = 0;
            }
            KeyCode::Char('j') | KeyCode::Down => app.tab2.next_topic_qs(),
            KeyCode::Char('k') | KeyCode::Up => app.tab2.prev_topic_qs(),
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('g') {
                        app.tab2.first_topic_qs();
                    }
                }
            }
            KeyCode::Char('G') => app.tab2.last_topic_qs(),
            KeyCode::Char('S') => {
                app.sync_state = true;
                app.tx
                    .send(UserEvent::StartSync(true))
                    .into_diagnostic()?;
            }
            KeyCode::Enter => app.goto_tab(1)?,
            KeyCode::Char('o') => {
                // stop listen keyevent
                *app.editor_flag.lock().unwrap() = false;
                app.tab2
                    .confirm_filtered_qs()
                    .await?;
                // start listen keyevent
                *app.editor_flag.lock().unwrap() = true;
                app.editor_cond.notify_one();
                app.get_code(&app.tab0.cur_qs.clone())
                    .await?;

                use crossterm::terminal::EnterAlternateScreen;
                execute!(stdout, EnterAlternateScreen).into_diagnostic()?;

                redraw(terminal, app)?;
            }
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
                app.tab2.filter_index = 1;
            }
            KeyCode::Char('k') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.filter_index = 0;
            }
            KeyCode::Char('l') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.filter_index = 2;
            }
            KeyCode::Char('j') | KeyCode::Down => app.tab2.next_user_topic(),
            KeyCode::Char('k') | KeyCode::Up => app.tab2.prev_user_topic(),
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('g') {
                        app.tab2.first_user_topic();
                    }
                }
            }
            KeyCode::Char('G') => app.tab2.last_user_topic(),
            KeyCode::Char('S') => {
                app.sync_state = true;
                app.tx
                    .send(UserEvent::StartSync(true))
                    .into_diagnostic()?;
            }
            // KeyCode::Enter => app.add_or_rm_user_topic().await,
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
                app.tab2.filter_index = 1;
            }
            KeyCode::Char('k') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.filter_index = 0;
            }
            KeyCode::Char('l') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.filter_index = 2;
            }
            KeyCode::Char('j') | KeyCode::Down => app.tab2.next_topic(),
            KeyCode::Char('k') | KeyCode::Up => app.tab2.prev_topic(),
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('g') {
                        app.tab2.first_topic();
                    }
                }
            }
            KeyCode::Char('G') => app.tab2.last_topic(),
            KeyCode::Char('S') => {
                app.sync_state = true;
                app.tx
                    .send(UserEvent::StartSync(true))
                    .into_diagnostic()?;
            }
            KeyCode::Enter => {
                app.tab2
                    .add_or_rm_user_topic()
                    .await;
            }
            _ => common_keymap(app, terminal, event, stdout).await?,
        },
        _ => common_keymap(app, terminal, event, stdout).await?,
    }

    Ok(())
}
