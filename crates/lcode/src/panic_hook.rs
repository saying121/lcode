use std::panic;

use lcode_config::config::global;
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry,
};

use crate::mytui::term::Term;

/// Set the panic hook to log panic information
pub fn init_panic_hook() {
    panic::set_hook(Box::new(|panic| {
        let appender = rolling::never(&*global::G_CACHE_DIR, global::LOG_FILE);
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
            .with(formatting_layer)
            .with(file_layer)
            .init();

        tracing::error!("Panic Error: {}", panic);

        Term::stop().expect("term stop failed");

        panic::take_hook()(panic);
    }));
    tracing::debug!("Set panic hook");
}
