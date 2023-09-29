pub mod app;
mod helper;
mod keymaps;
pub mod myevent;
mod ui;

use std::{
    io::{self, Stdout},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Condvar,
    },
    thread,
    time::Duration,
};

use crossterm::{
    event::{DisableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use miette::{IntoDiagnostic, Result};
use myevent::*;
use ratatui::{prelude::*, Terminal};
use tokio::sync::Mutex;
use tracing::error;

use crate::{
    config::global::glob_leetcode,
    dao::query_all_index,
    leetcode::{
        resps::{self, run_res::RunResult},
        IdSlug,
    },
};

use self::{app::*, ui::start_ui};

fn redraw<B: Backend>(terminal: &mut Terminal<B>, _app: &mut App) -> Result<()> {
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

    let (tx, rx) = mpsc::channel();

    let flag = Arc::new(std::sync::Mutex::new(true));
    let cond = Arc::new(Condvar::new());

    let events = Events::new(
        Duration::from_secs_f64(1.0 / 60.0),
        flag.clone(),
        cond.clone(),
    );
    let app = Arc::new(Mutex::new(App::new(tx.clone(), flag, cond).await));
    let eve_tx = events._tx.clone();

    let appclone = app.clone();
    thread::spawn(move || {
        block_oper(rx, eve_tx, appclone);
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
async fn block_oper(
    rx: Receiver<UserEvent>,
    eve_tx: Sender<UserEvent>,
    _app: Arc<Mutex<App>>,
) {
    while let Ok(event) = rx.recv() {
        match event {
            UserEvent::StartSync(idx) => {
                if let Err(err) = if idx {
                    glob_leetcode()
                        .new_sync_index()
                        .await
                } else {
                    glob_leetcode()
                        .sync_problem_index()
                        .await
                } {
                    error!("{}", err)
                }

                eve_tx
                    .send(UserEvent::SyncDone)
                    .unwrap_or_default();
            }
            UserEvent::GetQs((idslug, force)) => {
                let qs = glob_leetcode()
                    .get_qs_detail(idslug, force)
                    .await
                    .unwrap_or_default();
                let _ = eve_tx
                    .send(UserEvent::GetQsDone(qs))
                    .into_diagnostic();
            }
            UserEvent::SubmitCode(id) => {
                let mut temp: (resps::SubmitInfo, RunResult) =
                    (resps::SubmitInfo::default(), RunResult::default());
                // min id is 1
                if id > 0 {
                    temp = glob_leetcode()
                        .submit_code(IdSlug::Id(id))
                        .await
                        .unwrap_or_default();
                }
                let _ = eve_tx
                    .send(UserEvent::SubmitDone(temp.1))
                    .into_diagnostic();
            }
            UserEvent::TestCode(id) => {
                let mut temp: (resps::TestInfo, RunResult) =
                    (resps::TestInfo::default(), RunResult::default());
                // min id is 1
                if id > 0 {
                    temp = glob_leetcode()
                        .test_code(IdSlug::Id(id))
                        .await
                        .unwrap_or_default();
                }
                let _ = eve_tx
                    .send(UserEvent::TestDone(temp.1))
                    .into_diagnostic();
            }
            _ => {}
        }
    }
}

async fn run_inner<'run_lf, B: Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<App<'run_lf>>>,
    stdout: &mut Stdout,
    events: Events,
) -> Result<(), miette::ErrReport> {
    loop {
        let mut app = app.lock().await;
        terminal
            .draw(|f| start_ui(f, &mut app))
            .into_diagnostic()?;

        match events.next()? {
            UserEvent::SubmitDone(s_res) => {
                app.submit_res = s_res;
                app.show_submit_res = true;
                app.submiting = false;
            }
            UserEvent::TestDone(t_res) => {
                app.test_res = t_res;
                app.show_test_res = true;
                app.submiting = false;
            }
            UserEvent::GetQsDone(qs) => {
                match app.get_code(&qs).await {
                    Ok(_) => app.cur_qs = qs,
                    Err(err) => error!("Err: {}, try re-sync database", err),
                };
            }
            UserEvent::Syncing((cur_perc, title)) => {
                app.cur_perc = cur_perc;
                app.sync_title = title;
            }
            UserEvent::SyncDone => {
                app.sync_state = false;
                let questions = query_all_index()
                    .await
                    .unwrap_or_default();
                app.questions = questions.clone();
                app.questions_filtered = questions;
            }
            UserEvent::TermEvent(event) => match event {
                Event::Resize(_width, _height) => redraw(terminal, &mut app)?,
                Event::Key(keyevent) => match keyevent.code {
                    KeyCode::Char('c') if keyevent.modifiers == KeyModifiers::CONTROL => {
                        return Ok(())
                    }
                    KeyCode::Char('q') if keyevent.modifiers == KeyModifiers::CONTROL => {
                        return Ok(())
                    }
                    // KeyCode::Char('p') if keyevent.modifiers == KeyModifiers::CONTROL => {
                    //     app.pop_temp = !app.pop_temp;
                    // }
                    _ => match app.tab_index {
                        0 => {
                            keymaps::tab0::tab0_keymap(
                                &mut app, terminal, &event, stdout,
                            )
                            .await?;
                        }
                        1 => {
                            keymaps::tab1::init(&mut app, terminal, &event, stdout)
                                .await?;
                        }
                        2 => {
                            keymaps::tab2::init(&mut app, terminal, &event, stdout)
                                .await?;
                        }
                        3 => {
                            keymaps::tab3::init(&mut app, terminal, &event, stdout)
                                .await?;
                        }
                        _ => {}
                    },
                },
                _ => {}
            },
            _ => {}
        };
    }
}
