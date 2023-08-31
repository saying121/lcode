use lcode::cookies::get_cookie;
use miette::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt, Registry,
};

#[tokio::test]
async fn get_cookie_work() -> Result<()> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    let formatting_layer = fmt::layer()
        .pretty()
        .with_writer(std::io::stderr);
    Registry::default()
        .with(env_filter)
        .with(ErrorLayer::default())
        .with(formatting_layer)
        .init();
    let edge = get_cookie("edge").await?;
    println!(r##"(| edge |) -> {:#?}"##, edge);

    let chrome = get_cookie("chrome").await?;
    println!(r##"(| chrome |) -> {:#?}"##, chrome);

    let ff = get_cookie("firefox").await?;
    println!(r##"(| ff |) -> {:#?}"##, ff);

    let librewolf = get_cookie("librewolf").await?;
    println!(r##"(| librewolf |) -> {:#?}"##, librewolf);

    Ok(())
}
