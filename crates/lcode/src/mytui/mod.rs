mod app;
mod helper;
mod my_widget;
mod myevent;
mod next_key;
pub mod term;
mod ui;

use crossterm::event::Event;
use miette::Result;
use myevent::*;

use self::{
    app::{inner::App, *},
    term::Term,
    ui::start_ui,
};

pub async fn run() -> Result<()> {
    let mut terminal = Term::new()?;
    Term::start().ok();

    let events = EventsHandler::new();
    let mut app = App::new(events).await;

    app.render();

    while let Some(event) = app.events.next().await {
        match event {
            UserEvent::Quit => {
                app.events.stop_events()?;
                Term::stop()?;
                break;
            },
            UserEvent::SubmitDone(s_res) => app.submit_done(*s_res),
            UserEvent::TestDone(t_res) => app.test_done(*t_res),
            UserEvent::GetQsDone(qs) => app.get_qs_done(*qs).await,
            UserEvent::Syncing(cur_perc) => app.select.update_percent(cur_perc),
            UserEvent::SyncingNew(cur_perc) => app.topic.update_percent(cur_perc),
            UserEvent::SyncDone => app.sync_done().await,
            UserEvent::SyncDoneNew => app.sync_new_done().await,
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
        }
    }

    Ok(())
}
