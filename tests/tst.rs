use lcode::editor::{self, edit};
use lcode::leetcode::IdSlug;
use miette::Result;

use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt, Registry,
};

#[tokio::test]
async fn query_question_work() -> Result<()> {
    use lcode::{dao, leetcode::IdSlug};
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
    let a = dao::get_question_index(IdSlug::Id(0)).await?;
    println!(r##"(| 0 a |) -> {:#?}"##, a);
    let a = lcode::dao::get_question_index_exact(&IdSlug::Id(1)).await?;
    println!(r##"(| a |) -> {:#?}"##, a);

    Ok(())
}

#[tokio::test]
async fn edit_work() -> Result<()> {
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

    edit(IdSlug::Id(1000570), editor::CodeTestFile::Code).await?;
    edit(IdSlug::Id(1000570), editor::CodeTestFile::Test).await?;

    Ok(())
}
