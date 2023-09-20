use lcode::{config::global::glob_leetcode, leetcode::IdSlug, render::*};
use miette::{Error, Result};

use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt, Registry,
};

#[tokio::test]
async fn get_code_work() -> Result<()> {
    let a = glob_leetcode()
        .get_user_code(IdSlug::Id(108))
        .await?;
    println!(r##"(| a |) -> {:?}"##, a.0);
    println!(r##"(| a |) -> {}"##, a.0);
    Ok(())
}

#[tokio::test]
async fn new_get_index() -> Result<()> {
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
    let a = glob_leetcode();
    a.new_sync_index().await?;
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

    let a = glob_leetcode();
    if let Ok((_, res)) = a.test_code(IdSlug::Id(235)).await {
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

    let a = glob_leetcode();
    let question = a
        .get_qs_detail(IdSlug::Id(1), false)
        .await?;
    println!(r##"(| qsdetail |) -> {:#?}"##, question);
    let question = a
        .get_qs_detail(IdSlug::Slug("two-sum".to_owned()), false)
        .await?;
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

    let a = glob_leetcode();
    let question = a
        .get_qs_detail(IdSlug::Id(0), false)
        .await
        .unwrap();
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
    glob_leetcode()
        .sync_problem_index()
        .await?;
    Ok(())
}
// {
//     elapsed_time: 2078,
//     finished: true,
//     task_name: "judger.judgetask.Judge",
//     status_code: 14,
//     status_msg: "Time Limit Exceeded",
//     question_id: "45",
//     std_output: "",
//     expected_output: "2",
//     last_testcase: "[2,3,1,1,4]",
//     code_answer: [],
//     compare_result: "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
//     correct_answer: false,
//     expected_code_answer: [],
//     expected_code_output: [],
//     pretty_lang: "Rust",
//     lang: "rust",
//     memory: 2020000,
//     status_memory: "N/A",
//     memory_percentile: None,
//     status_runtime: "N/A",
//     runtime_percentile: None,
//     run_success: false,
//     state: "SUCCESS",
//     std_output_list: [],
//     submission_id: "468075289",
//     task_finish_time: 1695200759312,
//     total_correct: Some(
//         0,
//     ),
//     total_testcases: Some(
//         109,
//     ),
//     full_runtime_error: "",
//     runtime_error: "",
//     compile_error: "",
//     full_compile_error: "",
// }

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

    let a = glob_leetcode();
    let (_, res) = a
        .submit_code(IdSlug::Id(45))
        .await?;
    println!(r##"(| res |) -> {} "##, res);
    render_str(res.to_string())?;

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

    let a = glob_leetcode();
    let res = a
        .all_submit_res(IdSlug::Id(1))
        .await?;
    println!("{}", res);
    // render_str(res.to_string())?;
    // let res = get_rendered_str(res.to_string(), 30, 10)?;
    // println!("{}", res);

    Ok(())
}
