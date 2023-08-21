use lcode::cli::*;
use lcode::config::global::global_log_dir;
use miette::Result;
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
    Registry,
};

#[tokio::main]
async fn main() -> Result<()> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("error"));
    let formatting_layer = fmt::layer()
        .pretty()
        .with_writer(std::io::stderr);

    let log_dir = global_log_dir();

    let file_appender = rolling::never(log_dir, "app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_ansi(true)
        .with_writer(non_blocking_appender);

    Registry::default()
        .with(env_filter)
        .with(ErrorLayer::default())
        .with(formatting_layer)
        .with(file_layer)
        .init();

    run().await?;

    Ok(())
}
