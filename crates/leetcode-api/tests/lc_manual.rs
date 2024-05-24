use leetcode_api::{glob_leetcode, leetcode::IdSlug, render::Render};
use miette::Result;

#[ignore = "manual"]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    match glob_leetcode()
        .await
        .test_code(IdSlug::Id(435))
        .await
    {
        Ok((_, res)) => {
            dbg!(&res);
            println!(r##"(| res |) -> {} "##, res.to_md_str(false));
            // res.render_with_mdcat();
            // dbg!(res.to_tui_vec());
        },
        Err(e) => eprintln!("{e}"),
    }

    Ok(())
}

#[ignore = "need realy environment"]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn submit_work() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let (_, res) = glob_leetcode()
        .await
        .submit_code(IdSlug::Id(27))
        .await
        .unwrap();
    dbg!(res.to_para_vec());
    println!(r##"(| res |) -> {} "##, res.to_md_str(false));
    res.render_with_mdcat();
}

#[ignore = "manual"]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_submit_list() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let a = glob_leetcode().await;
    let res = a.all_submit_res(IdSlug::Id(32)).await?;
    println!("{}", res);
    // render_str(res.to_string())?;
    // let res = get_rendered_str(res.to_string(), 30, 10)?;
    // println!("{}", res);

    Ok(())
}
#[ignore = "manual"]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn daily_checkin() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let a = glob_leetcode().await;
    let res = a.daily_checkin().await;
    println!("{:#?}", res);

    Ok(())
}
#[ignore = "manual"]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn user_points() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let a = glob_leetcode().await;
    let res = a.get_points().await?;
    println!("{:#?}", res);

    Ok(())
}
#[ignore = "manual"]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn user_global_data() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let a = glob_leetcode().await;
    let res = a.get_user_info().await?;
    println!("{:#?}", res);

    Ok(())
}

#[ignore = "manual"]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn user_pass_data() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let a = glob_leetcode().await;
    let temp = a.get_user_info().await?;
    let res = a
        .pass_qs_status(&temp.user_slug.unwrap_or_default())
        .await?;
    println!("{:#?}", res);

    Ok(())
}
