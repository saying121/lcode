pub mod app;
pub(self) mod helper;
pub mod myevent;
mod ui;
mod term;

use std::{
    io::{self, Stdout},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use crossterm::{
    event::{
        self, DisableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use miette::{IntoDiagnostic, Result};
use myevent::*;
use ratatui::{prelude::*, Terminal};
use tui_input::backend::crossterm::EventHandler;

use crate::{
    config::global::global_leetcode, leetcode::question_detail::Question,
    storage::query_question::query_all_index,
};

use self::app::*;
use self::{helper::panic_hook::init_panic_hook, ui::start_ui};

fn redraw<B: Backend>(terminal: &mut Terminal<B>, _app: &mut App) -> Result<()> {
    terminal
        .resize(terminal.size().into_diagnostic()?)
        .into_diagnostic()?;
    // terminal
    //     .draw(|f| start_ui(f, _app))
    //     .into_diagnostic()?;
    Ok(())
}

pub async fn run() -> Result<()> {
    init_panic_hook();
    // setup terminal
    let mut stdout = io::stdout();
    enable_raw_mode().into_diagnostic()?;
    execute!(stdout, EnterAlternateScreen).into_diagnostic()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).into_diagnostic()?;

    let (tx, rx) = mpsc::channel();
    let events = Events::new(Duration::from_secs_f64(1.0 / 60.0));
    // let events = Events::new(Duration::from_millis(200));
    let app = Arc::new(Mutex::new(App::new(tx.clone()).await));
    let eve_tx = events._tx.clone();

    thread::spawn(move || {
        block_oper(rx, eve_tx);
    });

    run_inner(&mut terminal, app, &mut stdout, events).await?;

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

#[tokio::main]
async fn block_oper<'a>(rx: Receiver<UserEvent>, eve_tx: Sender<UserEvent>) {
    while let Ok(event) = rx.recv() {
        match event {
            _ => {
                let lcd = global_leetcode();
                if let Err(err) = lcd
                    .sync_index_with_state(eve_tx.clone())
                    .await
                {
                    eprintln!("{}", err);
                }
            }
        }
    }
}

async fn run_inner<'a, B: Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<App<'a>>>,
    stdout: &mut Stdout,
    events: Events,
) -> Result<(), miette::ErrReport> {
    let app = &mut app.lock().unwrap();
    loop {
        terminal
            .draw(|f| start_ui(f, app))
            .into_diagnostic()?;

        match events.next()? {
            UserEvent::Syncing((cur, total, title)) => {
                app.cur_index_num = cur;
                app.total_index_num = total;
                app.sync_title = title;
            }
            UserEvent::SyncDone => {
                app.sync_state = false;

                let questions = query_all_index()
                    .await
                    .unwrap_or_default();
                app.questions = questions;
            }
            UserEvent::InputKey(key) => match key {
                KeyEvent {
                    code,
                    modifiers,
                    kind,
                    state: _,
                } => match kind {
                    KeyEventKind::Press => {
                        mod_keymap(app, terminal, modifiers, code)?;
                        match code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::BackTab | KeyCode::Left => app.prev_tab(),
                            KeyCode::Tab | KeyCode::Right => {
                                app.next_tab();
                                if app.tab_index == 1 {
                                    let lcd = global_leetcode();
                                    let qs_id = app.current_qs();
                                    app.cur_qs = if qs_id <= 0 {
                                        Question::default()
                                    } else {
                                        lcd.get_problem_detail(
                                            crate::leetcode::IdSlug::Id(qs_id),
                                            false,
                                        )
                                        .await
                                        .unwrap_or_default()
                                    };
                                }
                            }
                            _ => match app.tab_index {
                                0 => {
                                    tab0_keymap(app, terminal, key, stdout).await?;
                                }
                                1 => {
                                    tab1_keymap(app, terminal, key, stdout)?;
                                }
                                _ => {}
                            },
                        }
                    }
                    _ => {}
                },
            },
            UserEvent::TermEvent(Event::Resize(_width, _height)) => {
                redraw(terminal, app)?
            }
            _ => {}
        };
    }
}

fn mod_keymap<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    modifiers: KeyModifiers,
    code: KeyCode,
) -> Result<()> {
    match modifiers {
        KeyModifiers::CONTROL => match code {
            KeyCode::Char('r') => redraw(terminal, app)?,
            KeyCode::Char('c') => return Ok(()),
            _ => {}
        },
        _ => {}
    }
    Ok(())
}

async fn tab0_keymap<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    key: KeyEvent,
    stdout: &mut Stdout,
) -> Result<()> {
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('e') => {
                app.input_mode = InputMode::Editing;
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
                            KeyCode::Char('g') => app.first(),
                            _ => {}
                        }
                    }
                }
            }
            KeyCode::Char('G') => app.last(),
            KeyCode::Enter => app.next_tab(),
            KeyCode::Down | KeyCode::Char('j') => app.next(),
            KeyCode::Up | KeyCode::Char('k') => app.previous(),
            KeyCode::Char('o') => {
                execute!(stdout, LeaveAlternateScreen).into_diagnostic()?;
                app.confirm().await?;
                execute!(stdout, EnterAlternateScreen).into_diagnostic()?;

                // redraw
                redraw(terminal, app)?;
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
    };
    Ok(())
}

fn tab1_keymap<B: Backend>(
    app: &mut App,
    _terminal: &mut Terminal<B>,
    key: KeyEvent,
    _stdout: &mut Stdout,
) -> Result<()> {
    match key.code {
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
        KeyCode::Char('g') => {
            if let Event::Key(key) = event::read().into_diagnostic()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('g') => {
                            app.vertical_scroll = 0;
                            app.vertical_scroll_state = app
                                .vertical_scroll_state
                                .position(app.vertical_scroll as u16);
                        }
                        _ => {}
                    }
                }
            }
        }
        KeyCode::Char('G') => {
            app.vertical_scroll = app
                .vertical_row_len
                .saturating_sub(4);
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
    }

    Ok(())
}
