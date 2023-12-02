use std::{
    sync::{
        atomic::Ordering,
        mpsc::{channel, Receiver, Sender},
        Arc, Condvar, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    self,
    event::{self, Event},
};
use miette::{IntoDiagnostic, Result};

use crate::leetcode::{
    qs_detail::Question, resps::run_res::RunResult, IdSlug, CUR_QS_INDEX_NUM, CUR_NEW_QS_INDEX_NUM, TOTAL_QS_INDEX_NUM,
    TOTAL_NEW_QS_INDEX_NUM,
};

pub enum UserEvent {
    TermEvent(Event),
    // Tick,
    GetQs((IdSlug, bool)), // id, and force or not
    GetQsDone(Question),
    Syncing(f64),
    SyncDone,
    SyncingNew(f64),
    SyncDoneNew,

    SubmitCode(u32),
    SubmitDone(RunResult),
    TestCode(u32),
    TestDone(RunResult),
}

pub struct Events {
    pub rx: Receiver<UserEvent>,
    pub tx: Sender<UserEvent>,
    pub is_shutdown: bool,
}

impl Events {
    pub fn new(tick_rate: Duration, flag: Arc<Mutex<bool>>, cond: Arc<Condvar>) -> Self {
        let (tx, rx) = channel();
        let event_tx = tx.clone();

        let mut last_tick = Instant::now();
        let mut last_tick_progress = Instant::now();

        thread::spawn(move || loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            let mut flag_v;
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

            let tot: f64 = TOTAL_QS_INDEX_NUM
                .load(Ordering::Acquire)
                .try_into()
                .unwrap_or_default();

            if tot > 0.0 && last_tick_progress.elapsed() > Duration::from_secs(1) {
                last_tick_progress = Instant::now();
                let cur = CUR_QS_INDEX_NUM.load(Ordering::Acquire);
                let cur: f64 = cur.try_into().unwrap_or_default();
                event_tx
                    .send(UserEvent::Syncing(cur / tot))
                    .expect("send error");
            }

            let tot: f64 = TOTAL_NEW_QS_INDEX_NUM
                .load(Ordering::Acquire)
                .try_into()
                .unwrap_or_default();
            if tot > 0.0 && last_tick_progress.elapsed() > Duration::from_secs(1) {
                last_tick_progress = Instant::now();
                let cur = CUR_NEW_QS_INDEX_NUM.load(Ordering::Acquire);
                let cur: f64 = cur.try_into().unwrap_or_default();
                event_tx
                    .send(UserEvent::SyncingNew(cur / tot))
                    .expect("send error");
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        });

        Self {
            rx,
            tx,
            is_shutdown: false,
        }
    }

    pub fn next(&self) -> Result<UserEvent> {
        self.rx.recv().into_diagnostic()
    }
}
