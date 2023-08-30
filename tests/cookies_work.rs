use tracing_subscriber::util::SubscriberInitExt;
use lcode::cookies::{
    chromium_base,
    ff_base::{get_ff_session_csrf, FIREFOX_LINUX, LIBREWOLF_LINUX},
    get_cookie,
};
use miette::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

#[tokio::test]
async fn get_cookie_work() -> Result<()> {
    let edge = get_cookie("edge").await?;
    println!(r##"(| edge |) -> {:#?}"##, edge);

    let ff = get_cookie("firefox").await?;
    println!(r##"(| ff |) -> {:#?}"##, ff);

    let librewolf = get_cookie("librewolf").await?;
    println!(r##"(| librewolf |) -> {:#?}"##, librewolf);

    Ok(())
}

#[tokio::test]
async fn get_cookie_work_edge() -> Result<()> {
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
    println!("1111");
    let cookie = chromium_base::get_chrom_session_csrf().await?;
    println!(r##"(| cookie |) -> {:#?}"##, cookie);

    Ok(())
}
#[tokio::test]
async fn get_cookie_work_ff() -> Result<()> {
    let res = get_ff_session_csrf(LIBREWOLF_LINUX).await?;
    println!(r##"(| res |) -> {:#?}"##, res);
    let res = get_ff_session_csrf(FIREFOX_LINUX).await?;
    println!(r##"(| res |) -> {:#?}"##, res);

    Ok(())
}
