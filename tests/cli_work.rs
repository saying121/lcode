use lcode::leetcode::IdSlug;
use lcode::{config::global::glob_leetcode, fuzzy_search::select_a_question, render::*};

use miette::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt, Registry,
};
use unicode_width::UnicodeWidthStr;

#[test]
fn width() {
    let a = "剑指 Offer 32 - III";
    let b = "a";
    println!("{:17}|", a);
    println!("{:19}|", b);

    let a = "剑指 Offer 10- II";
    let wd = UnicodeWidthStr::width(a);
    println!("{}", wd);
    let a = "II";
    let wd = UnicodeWidthStr::width(a);
    println!("{}", wd);
}

#[tokio::test]
async fn select_work() -> Result<()> {
    let id = select_a_question().await?;
    if id == 0 {
        return Ok(());
    }
    println!("{}", id);

    let a = glob_leetcode();
    let qs = a
        .get_qs_detail(IdSlug::Id(id), false)
        .await?;
    render_qs_to_tty(qs)?;
    Ok(())
}

#[tokio::test]
async fn index_display_work() -> Result<()> {
    use lcode::dao;
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

    let idx = dao::query_all_index().await?;
    println!("{:#?}", idx[1]);
    for i in 0..5 {
        println!("{}", idx[i]);
    }
    let length = idx.len();
    println!("{}", idx[length - 1]);
    println!("{}", idx[length - 2]);
    println!("{}", idx[length - 3]);

    Ok(())
}
