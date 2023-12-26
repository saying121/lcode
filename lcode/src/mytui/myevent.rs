use crossterm::event::{Event, EventStream, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::{
    select,
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use tracing::error;

use crate::leetcode::{qs_detail::Question, resps::run_res::RunResult};

pub enum UserEvent {
    TermEvent(Event),
    GetQsDone(Box<Question>),
    Syncing(f64),
    SyncDone,
    SyncingNew(f64),
    SyncDoneNew,

    SubmitCode(u32),
    SubmitDone(Box<RunResult>),
    TestCode(u32),
    TestDone(Box<RunResult>),

    Quit,

    Render,
}

pub struct EventsHandler {
    pub tx: mpsc::UnboundedSender<UserEvent>,
    pub rx: mpsc::UnboundedReceiver<UserEvent>,

    pub tx_end_event: Option<oneshot::Sender<()>>,
    // pub rx_end_term: Option<oneshot::Receiver<()>>,
    pub is_shutdown:  bool,

    pub task: Option<JoinHandle<()>>,
}

impl Drop for EventsHandler {
    fn drop(&mut self) {
        _ = self.stop();
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
            is_shutdown: false,

            task: Some(task),
        }
    }

    pub async fn next(&mut self) -> Option<UserEvent> {
        self.rx.recv().await
    }
    pub fn stop(&mut self) -> miette::Result<()> {
        if let Some(tx) = self.tx_end_event.take() {
            tx.send(())
                .map_err(|_| miette::miette!("stop send err"))?;
        }
        Ok(())
    }
}
