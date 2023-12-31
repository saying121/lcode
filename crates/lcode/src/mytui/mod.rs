mod app;
mod helper;
mod my_widget;
mod myevent;
mod term;
mod ui;

use crossterm::event::Event;
use miette::Result;
use myevent::*;

use self::{app::*, ui::start_ui};

#[derive(Default, Debug, PartialEq, Eq)]
pub enum TuiMode {
    /// input panel
    Normal,
    /// input panel
    Insert,
    /// input panel
    Select,

    /// not enter input
    #[default]
    OutEdit,
}

pub async fn run() -> Result<()> {
    let mut terminal = term::Term::start()?;

    let mut events = EventsHandler::new();
    let mut app = App::new(events.tx.clone()).await;

    terminal
        .draw(|f| start_ui(f, &mut app))
        .expect("Tui error");

    while let Some(event) = events.next().await {
        match event {
            UserEvent::Quit => {
                events.stop()?;
                terminal.stop()?;
                break;
                // app.stop()?;
            },
            UserEvent::SubmitDone(s_res) => app.tab1.submit_done(*s_res),
            UserEvent::TestDone(t_res) => app.tab1.test_done(*t_res),
            UserEvent::GetQsDone(qs) => app.get_qs_done(*qs).await,
            UserEvent::Syncing(cur_perc) => app.tab0.update_percent(cur_perc),
            UserEvent::SyncingNew(cur_perc) => app.tab2.update_percent(cur_perc),
            UserEvent::SyncDone => app.tab0.sync_done().await,
            UserEvent::SyncDoneNew => app.tab2.sync_new_done().await,
            UserEvent::Render => {
                terminal
                    .draw(|f| start_ui(f, &mut app))
                    .expect("Tui error");
            },
            UserEvent::TermEvent(event) => match event {
                Event::Key(keyevent) => app.handle_key(keyevent).await,
                Event::Resize(_width, _height) => terminal.redraw()?,
                Event::FocusGained => {},
                Event::FocusLost => {},
                Event::Mouse(_) => {},
                Event::Paste(_) => {},
            },
            _ => {},
        }
    }

    Ok(())
}
