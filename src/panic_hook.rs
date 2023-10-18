use std::panic;

use crossterm::{execute, terminal::disable_raw_mode};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
    Registry,
};

use crate::config::global;

/// Set the panic hook to log panic information
pub fn init_panic_hook() {
    panic::set_hook(Box::new(|panic| {
        let appender =
            tracing_appender::rolling::never(global::glob_log_dir(), "lcode.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        let file_layer = fmt::layer()
            .with_thread_ids(true)
            .with_level(true)
            .with_ansi(false)
            .with_writer(non_blocking);

        let formatting_layer = fmt::layer()
            .pretty()
            .with_writer(std::io::stderr);

        Registry::default()
            .with(env_filter)
            .with(ErrorLayer::default())
            .with(formatting_layer)
            .with(file_layer)
            .init();

        tracing::error!("Panic Error: {}", panic);

        disable_raw_mode().expect("Could not disable raw mode");
        execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)
            .expect("Could not leave the alternate screen");

        panic::take_hook()(panic);
    }));
    tracing::debug!("Set panic hook");
}
