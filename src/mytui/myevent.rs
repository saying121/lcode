use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Condvar, Mutex,
};
use std::{
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    self,
    event::{self, Event},
};
use miette::{IntoDiagnostic, Result};

use crate::leetcode::{
    qs_detail::Question,
    resps::{SubmissionDetail, TestResult},
};

pub enum UserEvent {
    TermEvent(Event),
    StartSync,
    SyncDone,
    Tick,
    GetQs(u32),
    GetQsDone(Question),
    Syncing((usize, usize, String)),
    SubmitCode,
    SubmitDone(SubmissionDetail),
    TestCode,
    TestDone(TestResult),
}

pub struct Events {
    pub rx: Receiver<UserEvent>,
    pub _tx: Sender<UserEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration, flag: Arc<Mutex<bool>>, cond: Arc<Condvar>) -> Self {
        let (tx, rx) = channel();
        let event_tx = tx.clone();

        let mut last_tick = Instant::now();

        thread::spawn(move || loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            // let mut flag_v = flag.try_lock();
            #[allow(unused_assignments)]
            let mut flag_v = true;
            if let Ok(v) = flag.try_lock() {
                flag_v = *v;
            } else {
                flag_v = true;
            }

            while !flag_v {
                flag_v = *cond
                    .wait(flag.lock().unwrap())
                    .unwrap();
            }

            if crossterm::event::poll(timeout).unwrap_or_default() {
                if let Ok(event) = event::read() {
                    event_tx
                        .send(UserEvent::TermEvent(event))
                        .expect("send event failed");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<UserEvent> {
        self.rx.recv().into_diagnostic()
    }
}
