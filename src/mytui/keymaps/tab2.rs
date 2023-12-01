use std::io::Stdout;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
};
use miette::{IntoDiagnostic, Result};
use ratatui::{prelude::Backend, Terminal};
use tui_textarea::{Input, Key};

use super::common_keymap;
use crate::mytui::{
    app::{App, InputMode, Tab2},
    redraw,
};

pub async fn init<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    match app.tab2.input_line_mode {
        InputMode::Normal => match app.tab2.index {
            Tab2::AllTopics => all_topic(app, terminal, event, stdout).await?,
            Tab2::UserTopics => user_topic(app, terminal, event, stdout).await?,
            Tab2::Difficulty => difficult(app, terminal, event, stdout).await?,
            Tab2::Questions => filtered_qs(app, terminal, event, stdout).await?,
        },
        InputMode::Insert => match event.clone().into() {
            Input { key: Key::Esc, .. } => app.tab2.be_input_normal(),
            input => {
                app.tab2.text_line.input(input);
            }
        },
    }
    Ok(())
}

async fn difficult<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    match event {
        Event::Key(keyevent) => match keyevent.code {
            KeyCode::Char('h') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.goto_all_topic();
            }
            KeyCode::Char('l') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.goto_filtered_qs();
            }
            KeyCode::Char('j') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.goto_user_topic();
            }
            KeyCode::Char('j') | KeyCode::Down => app.tab2.next_diff(),
            KeyCode::Char('k') | KeyCode::Up => app.tab2.prev_diff(),
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('g') {
                        app.tab2.first_diff();
                    }
                }
            }
            KeyCode::Char('G') => app.tab2.last_diff(),
            KeyCode::Enter => app.tab2.toggle_diff().await,
            _ => common_keymap(app, terminal, event, stdout).await?,
        },
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
            KeyCode::Char('e') => app.tab2.be_input_insert(),
            KeyCode::Char('j') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.goto_user_topic();
            }
            KeyCode::Char('k') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.goto_difficulty();
            }
            KeyCode::Char('h') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.goto_difficulty();
            }
            KeyCode::Char('j') | KeyCode::Down => app.tab2.next_qs(),
            KeyCode::Char('k') | KeyCode::Up => app.tab2.prev_qs(),
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('g') {
                        app.tab2.first_qs();
                    }
                }
            }
            KeyCode::Char('G') => app.tab2.last_qs(),
            KeyCode::Char('S') => app.sync_new()?,
            KeyCode::Enter => app.goto_tab(1)?,
            KeyCode::Char('o') => {
                app.stop_listen_key();

                app.tab2.edit_cur_qs().await?;

                app.start_listen_key();

                app.get_code(&app.cur_qs.clone())
                    .await?;

                use crossterm::terminal::EnterAlternateScreen;
                execute!(stdout, EnterAlternateScreen).into_diagnostic()?;

                redraw(terminal, app)?;
            }
            _ => common_keymap(app, terminal, event, stdout).await?,
        },
        _ => common_keymap(app, terminal, event, stdout).await?,
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
            KeyCode::Char('k') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.goto_all_topic();
            }
            KeyCode::Char('l') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.goto_difficulty();
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
            KeyCode::Char('S') => app.sync_new()?,
            KeyCode::Enter => app.tab2.rm_user_topic().await,
            _ => common_keymap(app, terminal, event, stdout).await?,
        },
        _ => common_keymap(app, terminal, event, stdout).await?,
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
                app.tab2.goto_user_topic();
            }
            KeyCode::Char('l') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab2.goto_difficulty();
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
            KeyCode::Char('S') => app.sync_new()?,
            KeyCode::Enter => app.tab2.add_user_topic().await,
            _ => common_keymap(app, terminal, event, stdout).await?,
        },
        _ => common_keymap(app, terminal, event, stdout).await?,
    }

    Ok(())
}
