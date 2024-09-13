use std::path::PathBuf;

use crossterm::event::{Event, EventStream, KeyEventKind};
use futures::{FutureExt, StreamExt};
use leetcode_api::leetcode::{
    question::qs_detail::Question,
    resps::{checkin::TotalPoints, pass_qs::PassData, run_res::RunResult, user_data::UserStatus},
};
use miette::Result;
use ratatui_image::protocol::StatefulProtocol;
use tokio::{
    select,
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use tracing::error;

#[derive(Clone)]
#[non_exhaustive]
pub enum UserEvent {
    TermEvent(Event),
    GetQsDone(Box<Question>),
    Syncing(f64),
    SyncDone,
    SyncingNew(f64),
    SyncDoneNew,

    SubmitDone(Box<RunResult>),
    TestDone(Box<RunResult>),

    UserInfo(Box<(UserStatus, TotalPoints, PassData, Option<PathBuf>)>),

    Quit,

    Render,
    RedrawImg(Box<dyn StatefulProtocol>),
}

#[derive(Debug)]
pub struct EventsHandler {
    pub tx: mpsc::UnboundedSender<UserEvent>,
    pub rx: mpsc::UnboundedReceiver<UserEvent>,

    pub tx_end_event: Option<oneshot::Sender<()>>,

    // pub rx_end_term: Option<oneshot::Receiver<()>>,
    pub task: Option<JoinHandle<()>>,
}

impl Default for EventsHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for EventsHandler {
    fn drop(&mut self) {
        if let Err(e) = self.stop_events() {
            error!("{e}");
        }
    }
}

impl EventsHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let (s, mut r) = oneshot::channel();

        let tx_cloned = tx.clone();
        let task = tokio::spawn(async move {
            let tx = tx_cloned;
            let mut reader = EventStream::new();
            loop {
                let next_event = reader.next().fuse();
                select! {
                    _ = &mut r => break,
                    Some(Ok(event)) = next_event => {
                        match event {
                            Event::Key(keyevent) => {
                                // just need send when press, sometime will have a `Release` kind
                                if keyevent.kind == KeyEventKind::Press {
                                    if let Err(err) = tx.send(UserEvent::TermEvent(Event::Key(keyevent))) {
                                        error!("{err}");
                                    }
                                }
                            },
                            event => {
                                if let Err(err) = tx.send(UserEvent::TermEvent(event)) {
                                    error!("{err}");
                                }
                            }
                        }
                    }
                }
            }
        });

        Self {
            tx,
            rx,
            tx_end_event: Some(s),

            task: Some(task),
        }
    }

    /// recv next event
    pub async fn next(&mut self) -> Option<UserEvent> {
        self.rx.recv().await
    }
    /// stop event stream
    pub fn stop_events(&mut self) -> Result<()> {
        if let Some(tx) = self.tx_end_event.take() {
            tx.send(())
                .map_err(|()| miette::miette!("stop send err"))?;
        }
        Ok(())
    }
    /// send info for render tui
    pub fn render(&self) {
        if let Err(err) = self.tx.send(UserEvent::Render) {
            error!("{err}");
        }
    }
    /// stop event stream, quit program
    pub fn exit(&mut self) -> bool {
        self.stop_events().ok();

        if let Err(err) = self.tx.send(UserEvent::Quit) {
            error!("{}", err);
        }
        false
    }
    pub fn redraw_tui(&self) {
        if let Err(e) = self
            .tx
            .send(UserEvent::TermEvent(crossterm::event::Event::Resize(1, 1)))
        {
            error!("{e}");
        }
    }
}
