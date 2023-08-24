use std::{fs, sync::Mutex};

use crossterm::{execute, terminal::disable_raw_mode};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::config::global;

pub fn init_panic_hook() {
    let mut dir = global::global_log_dir().clone();
    dir.push("apppanic.log");
    let log_file = {
        fs::create_dir_all(
            dir.parent()
                .expect("parent failed"),
        )
        .unwrap_or_default();
        fs::File::create(dir).expect("create file failed")
    };

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to output path.
        .with_max_level(Level::DEBUG)
        .with_writer(Mutex::new(log_file))
        .with_thread_ids(true)
        .with_ansi(true)
        .with_line_number(true);

    let subscriber = subscriber.finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Set the panic hook to log panic information before panicking
    std::panic::set_hook(Box::new(|panic| {
        let original_hook = std::panic::take_hook();
        tracing::error!("Panic Error: {}", panic);
        disable_raw_mode().expect("Could not disable raw mode");
        execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)
            .expect("Could not leave the alternate screen");

        original_hook(panic);
    }));
    tracing::debug!("Set panic hook")
}
