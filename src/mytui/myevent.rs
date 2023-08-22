use std::sync::mpsc::{channel, Receiver, Sender};
use std::{
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    self,
    event::{self, KeyEvent},
};
use miette::{IntoDiagnostic, Result};

use crate::leetcode::question_detail::Question;

pub enum UserEvent {
    InputKey(KeyEvent),
    TermEvent(event::Event),
    StartSync,
    SyncDone,
    Tick,
    GetQs(u32),
    GetQsDone(Question),
    Syncing((usize, usize, String)),
}

pub struct Events {
    pub rx: Receiver<UserEvent>,
    pub _tx: Sender<UserEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = channel();
        let event_tx = tx.clone();

        let mut last_tick = Instant::now();

        thread::spawn(move || loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout).unwrap_or_default() {
                if let Ok(event) = event::read() {
                    match event {
                        event::Event::Key(key) => {
                            event_tx
                                .send(UserEvent::InputKey(key))
                                .expect("send key event error");
                        }
                        event::Event::Resize(width, height) => event_tx
                            .send(UserEvent::TermEvent(event::Event::Resize(
                                width, height,
                            )))
                            .expect("send resize event error"),
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                // event_tx
                //     .send(UserEvent::Tick)
                //     .expect("send event error");
                last_tick = Instant::now();
            }
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<UserEvent> {
        self.rx.recv().into_diagnostic()
    }
}
