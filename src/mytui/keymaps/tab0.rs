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

pub async fn init<B: Backend>(
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
                        app.stop_listen_key();

                        edit_config().await?;

                        app.start_listen_key();

                        // let res = USER_CONFIG
                        //     .set(read_config::get_user_conf().unwrap_or_default());

                        use crossterm::terminal::EnterAlternateScreen;
                        execute!(stdout, EnterAlternateScreen).into_diagnostic()?;

                        redraw(terminal, app)?;
                    }
                    KeyCode::Char('e') => app.tab0.be_insert(),
                    KeyCode::Char('S') => app.sync_index()?,
                    KeyCode::Char('g') => {
                        if let Event::Key(key) = event::read().into_diagnostic()? {
                            if key.kind == KeyEventKind::Press
                                && key.code == KeyCode::Char('g')
                            {
                                app.tab0.first_qs();
                            }
                        }
                    }
                    KeyCode::Char('G') => app.tab0.last_qs(),
                    KeyCode::Enter => app.goto_tab(1)?,
                    KeyCode::Down | KeyCode::Char('j') => app.tab0.next_qs(),
                    KeyCode::Up | KeyCode::Char('k') => app.tab0.prev_qs(),
                    KeyCode::Char('r') if keyevent.modifiers == KeyModifiers::CONTROL => {
                        app.tx
                            .send(UserEvent::GetQs((
                                IdSlug::Id(app.tab0.current_qs()),
                                true,
                            )))
                            .into_diagnostic()?;
                    }
                    KeyCode::Char('o') => {
                        app.stop_listen_key();

                        app.tab0.confirm_qs().await?;

                        app.start_listen_key();

                        app.get_code(&app.cur_qs.clone())
                            .await?;

                        use crossterm::terminal::EnterAlternateScreen;
                        execute!(stdout, EnterAlternateScreen).into_diagnostic()?;

                        redraw(terminal, app)?;
                    }
                    _ => common_keymap(app, terminal, event, stdout).await?,
                }
            }
        }
        InputMode::Insert => match event.clone().into() {
            Input { key: Key::Esc, .. } => app.tab0.be_normal(),
            input => {
                app.tab0.text_line.input(input);
            }
        },
    };
    Ok(())
}
