use lcode::{config::global::global_leetcode, leetcode::IdSlug, render::*};
use miette::{Error, Result};

use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt, Registry,
};

#[tokio::test]
async fn new_get_index() -> Result<()> {
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
    let a = global_leetcode();
    let _a = a.new_sync_index().await?;
    Ok(())
}

#[tokio::test]
async fn test_work() -> Result<()> {
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

    let a = global_leetcode();
    if let Ok((_, res)) = a.test_code(IdSlug::Id(1)).await {
        println!(r##"(| res |) -> {} "##, res);
        render_str(res.to_string())?;
    }

    Ok(())
}

#[tokio::test]
async fn get_qs_detail_work() -> Result<(), Error> {
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

    let a = global_leetcode();
    let question = a.get_qs_detail(IdSlug::Id(404), false).await?;
    println!(r##"(| qsdetail |) -> {:#?}"##, question);

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn get_qs_detail_work1() {
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

    let a = global_leetcode();
    let question = a.get_qs_detail(IdSlug::Id(0), false).await.unwrap();
    println!(r##"(| qsdetail |) -> {:#?}"##, question);
}

#[tokio::test]
async fn get_all_pbs_works() -> Result<()> {
    // let env_filter =
    //     EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    // let formatting_layer = fmt::layer()
    //     .pretty()
    //     .with_writer(std::io::stderr);
    // Registry::default()
    //     .with(env_filter)
    //     .with(ErrorLayer::default())
    //     .with(formatting_layer)
    //     .init();
    global_leetcode().sync_problem_index().await?;
    Ok(())
}

#[tokio::test]
async fn submit_work() -> Result<()> {
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

    let a = global_leetcode();
    let res = a.submit_code(IdSlug::Id(1)).await;
    match res {
        Ok(v) => {
            let (_, res) = v;
            println!("{}", res);
            render_str(res.to_string())?;
        }
        Err(err) => println!("{}", err),
    };

    Ok(())
}

#[tokio::test]
async fn get_submit_list() -> Result<()> {
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

    let a = global_leetcode();
    let res = a.all_submit_res(IdSlug::Id(1)).await?;
    println!("{}", res);
    // render_str(res.to_string())?;
    // let res = get_rendered_str(res.to_string(), 30, 10)?;
    // println!("{}", res);

    Ok(())
}
