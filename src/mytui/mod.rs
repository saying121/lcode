mod app;
mod myevent;
mod ui;

use std::{
    io::{self, Stdout},
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use miette::{IntoDiagnostic, Result};
use myevent::*;
use ratatui::{prelude::*, Terminal};
use tui_input::backend::crossterm::EventHandler;

use crate::config::global::global_leetcode;

use self::app::*;
use self::ui::ui;

fn redraw(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    terminal
        .resize(terminal.size().into_diagnostic()?)
        .into_diagnostic()?;
    Ok(())
}

pub async fn run() -> Result<()> {
    // setup terminal
    let mut stdout = io::stdout();
    enable_raw_mode().into_diagnostic()?;
    execute!(stdout, EnterAlternateScreen).into_diagnostic()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).into_diagnostic()?;
    let mut app = App::new().await;

    run_inner(&mut terminal, &mut app, &mut stdout).await?;

    // restore terminal
    disable_raw_mode().into_diagnostic()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .into_diagnostic()?;

    terminal
        .show_cursor()
        .into_diagnostic()?;

    Ok(())
}

async fn run_inner<'a>(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App<'a>,
    stdout: &mut Stdout,
) -> Result<(), miette::ErrReport> {
    let events = Events::new(Duration::from_secs_f64(1.0 / 60.0));

    loop {
        terminal
            .draw(|f| ui(f, app))
            .into_diagnostic()?;

        match events.next()? {
            UserEvents::InputKey(key) => {
                match key.kind {
                    KeyEventKind::Press => {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::BackTab | KeyCode::Left => app.prev_tab(),
                            KeyCode::Tab | KeyCode::Right => app.next_tab(),
                            KeyCode::Char('r') => redraw(terminal)?,
                            KeyCode::Char('S') => {
                                let leetcode = global_leetcode();
                                let res = thread::spawn(move || {
                                    let rt =
                                        tokio::runtime::Builder::new_current_thread()
                                            .enable_all()
                                            .build()
                                            .expect("tokio runtime build failed");
                                    rt.block_on(leetcode.sync_problem_index())
                                })
                                .join()
                                .expect("sync db failed");
                                res?
                            }
                            _ => {
                                match app.tab_index {
                                    0 => {
                                        match app.input_mode {
                                            InputMode::Normal => match key.code {
                                                KeyCode::Char('e') => {
                                                    app.input_mode = InputMode::Editing;
                                                }
                                                KeyCode::Char('g') => {
                                                    if crossterm::event::poll(
                                                        Duration::from_millis(200),
                                                    )
                                                    .into_diagnostic()?
                                                    {
                                                        if let Event::Key(key) =
                                                            event::read()
                                                                .into_diagnostic()?
                                                        {
                                                            if key.kind
                                                                == KeyEventKind::Press
                                                            {
                                                                match key.code {
                                                                    KeyCode::Char(
                                                                        'g',
                                                                    ) => app.first(),
                                                                    _ => {}
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                KeyCode::Char('G') => app.last(),
                                                KeyCode::Down | KeyCode::Char('j') => {
                                                    app.next()
                                                }
                                                KeyCode::Up | KeyCode::Char('k') => {
                                                    app.previous()
                                                }
                                                KeyCode::Char('o') => {
                                                    execute!(
                                                        stdout,
                                                        LeaveAlternateScreen
                                                    )
                                                    .into_diagnostic()?;
                                                    app.confirm().await?;
                                                    execute!(
                                                        stdout,
                                                        EnterAlternateScreen
                                                    )
                                                    .into_diagnostic()?;

                                                    // redraw
                                                    redraw(terminal)?;
                                                }
                                                _ => {}
                                            },
                                            InputMode::Editing => match key.code {
                                                KeyCode::Enter => {
                                                    app.input.reset();
                                                }
                                                KeyCode::Esc => {
                                                    app.input_mode = InputMode::Normal;
                                                }
                                                _ => {
                                                    app.input
                                                        .handle_event(&Event::Key(key));
                                                }
                                            },
                                        }
                                    }
                                    1 => match key.code {
                                        KeyCode::Char('j') => {
                                            if app.vertical_scroll
                                                < app
                                                    .vertical_row_len
                                                    .saturating_sub(4)
                                            {
                                                app.vertical_scroll = app
                                                    .vertical_scroll
                                                    .saturating_add(1);
                                            }
                                            app.vertical_scroll_state = app
                                                .vertical_scroll_state
                                                .position(app.vertical_scroll as u16);
                                        }
                                        KeyCode::Char('k') => {
                                            app.vertical_scroll = app
                                                .vertical_scroll
                                                .saturating_sub(1);
                                            app.vertical_scroll_state = app
                                                .vertical_scroll_state
                                                .position(app.vertical_scroll as u16);
                                        }
                                        KeyCode::Char('h') => {
                                            app.horizontal_scroll = app
                                                .horizontal_scroll
                                                .saturating_sub(1);
                                            app.horizontal_scroll_state = app
                                                .horizontal_scroll_state
                                                .position(app.horizontal_scroll as u16);
                                        }
                                        KeyCode::Char('l') => {
                                            if app.horizontal_scroll
                                                < app
                                                    .horizontal_col_len
                                                    .saturating_sub(4)
                                            {
                                                app.horizontal_scroll = app
                                                    .horizontal_scroll
                                                    .saturating_add(1);
                                            }
                                            app.horizontal_scroll_state = app
                                                .horizontal_scroll_state
                                                .position(app.horizontal_scroll as u16);
                                        }
                                        _ => {}
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            UserEvents::TermEvent(Event::Resize(_width, _height)) => redraw(terminal)?,
            UserEvents::TermEvent(_) => {}
        };
    }
}
