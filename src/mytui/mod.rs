mod app;
mod helper;
mod keymaps;
mod myevent;
mod ui;

use std::{
    io::{self, Stdout},
    sync::{Arc, Condvar},
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
use ratatui::{prelude::*, Terminal};

use self::{app::*, ui::start_ui};
use myevent::*;

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

    #[allow(renamed_and_removed_lints)]
    #[allow(mutex_atomic)]
    let flag = Arc::new(std::sync::Mutex::new(true));
    let cond = Arc::new(Condvar::new());

    let events = Events::new(
        Duration::from_secs_f64(1.0 / 60.0),
        Arc::clone(&flag),
        Arc::clone(&cond),
    );
    let app = App::new(events.tx.clone(), flag, cond).await;

    Box::pin(run_inner(&mut terminal, app, &mut stdout, events)).await?;

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

async fn run_inner<'run_lf, B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App<'run_lf>,
    stdout: &mut Stdout,
    events: Events,
) -> Result<()> {
    loop {
        // let app = app.borrow_mut();
        terminal
            .draw(|f| start_ui(f, &mut app))
            .into_diagnostic()?;

        match events.next()? {
            UserEvent::SubmitDone(s_res) => app.tab1.submit_done(*s_res),
            UserEvent::TestDone(t_res) => app.tab1.test_done(*t_res),
            UserEvent::GetQsDone(qs) => app.get_qs_done(*qs).await,
            UserEvent::Syncing(cur_perc) => app.tab0.update_percent(cur_perc),
            UserEvent::SyncingNew(cur_perc) => app.tab2.update_percent(cur_perc),
            UserEvent::SyncDone => app.tab0.sync_done().await,
            UserEvent::SyncDoneNew => app.tab2.sync_new_done().await,
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
                            keymaps::tab0::init(&mut app, terminal, &event, stdout)
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
