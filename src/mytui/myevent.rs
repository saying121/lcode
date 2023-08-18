use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    self,
    event::{self, KeyEvent},
};
use miette::{IntoDiagnostic, Result};

pub enum UserEvents {
    InputKey(KeyEvent),
    TermEvent(event::Event),
}

pub struct Events {
    pub rx: Receiver<UserEvents>,
    pub _tx: Sender<UserEvents>,
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
                match event::read().unwrap() {
                    event::Event::Key(key) => {
                        event_tx
                            .send(UserEvents::InputKey(key))
                            .expect("send key event error");
                    }
                    event::Event::Resize(width, height) => event_tx.send(
                        UserEvents::TermEvent(event::Event::Resize(width, height)),
                    ).expect("send resize event error"),
                    _ => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<UserEvents> {
        self.rx.recv().into_diagnostic()
    }
}
