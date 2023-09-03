use lcode::{
    config::global::glob_leetcode,
    leetcode::IdSlug,
    render::{self, pre_render, Render},
};
use miette::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, fmt, Registry, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[tokio::test]
async fn render_html() -> Result<()> {
    let a = glob_leetcode();
    let qs = a
        .get_qs_detail(IdSlug::Id(1), false)
        .await?;

    println!("{:#?}", qs.to_tui_vec());

    Ok(())
}

#[tokio::test]
async fn render_md_terminal() -> Result<()> {
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
    let a = glob_leetcode();
    let id = 100483;
    let qs = a
        .get_qs_detail(IdSlug::Id(id), true)
        .await?;
    println!(r##"(| qs |) -> {:#?}"##, qs);

    use lcode::render::*;
    render_qs_to_tty(qs)?;

    Ok(())
}

#[tokio::test]
async fn render_md_str() -> Result<()> {
    let a = glob_leetcode();
    let id = 100092;
    println!("1");
    let qs = a
        .get_qs_detail(IdSlug::Id(id), false)
        .await?;
    println!(r##"(| qs |) -> {:#?}"##, qs);

    use render::Render;
    let a = qs.to_rendered_str(80, 80)?;
    println!("{}", a);

    Ok(())
}

#[tokio::test]
async fn pre() -> Result<()> {
    let a = glob_leetcode();
    let id = 100092;
    let qs = a
        .get_qs_detail(IdSlug::Id(id), false)
        .await?;
    let a = pre_render(&qs);
    println!("{}", a);

    Ok(())
}

#[tokio::test]
async fn render_md_str1() -> Result<()> {
    let a = glob_leetcode();
    let id = 100092;
    let qs = a
        .get_qs_detail(IdSlug::Id(id), false)
        .await?;
    println!(r##"(| qs |) -> {:#?}"##, qs);

    use render::Render;
    let a = qs.to_md_str();
    println!("{}", a);

    Ok(())
}
