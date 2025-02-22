pub mod my_widget;

mod helper;
pub mod myevent;
pub mod term;
mod ui;

use crossterm::event::Event;
use miette::Result;
use myevent::*;

use self::{term::Term, ui::start_ui};
use crate::app::{inner::App, *};

pub async fn run() -> Result<()> {
    let mut terminal = Term::new()?;
    Term::start().ok();

    let events = EventsHandler::new();
    let mut app = App::new(events).await;
    app.user_info_and_checkin();

    app.render();

    while let Some(event) = app.events.next().await {
        match event {
            UserEvent::Quit => {
                app.events.stop_events()?;
                Term::stop()?;
                break;
            },
            UserEvent::UserInfo(info) => app.get_status_done(*info),
            UserEvent::SubmitDone(s_res) => {
                // update info
                if s_res.total_correct == s_res.total_testcases {
                    app.user_info_and_checkin();
                }
                app.submit_done(*s_res);
            },
            UserEvent::TestDone(t_res) => app.test_done(*t_res),
            UserEvent::GetQsDone(qs) => app.get_qs_done(*qs).await,
            UserEvent::Syncing(perc) => app.select.update_percent(perc),
            UserEvent::SyncingNew(perc) => app.topic.update_percent(perc),
            UserEvent::SyncDone => app.sync_done().await,
            UserEvent::SyncDoneNew => app.sync_new_done().await,
            UserEvent::Render => {
                terminal
                    .draw(|f| start_ui(f, &mut app))
                    .expect("Tui error");
            },
            UserEvent::TermEvent(event) => match event {
                Event::Key(keyevent) => app.handle_key(keyevent).await,
                Event::Resize(width, height) => {
                    terminal.resize(width, height)?;
                    app.render();
                },
                Event::FocusGained | Event::FocusLost | Event::Mouse(_) | Event::Paste(_) => {},
            },
            UserEvent::RedrawImg(protocol) => {
                if let Some(state) = &mut app.img_state {
                    state.set_protocol(protocol);
                    app.render();
                }
            },
        }
    }

    Ok(())
}
