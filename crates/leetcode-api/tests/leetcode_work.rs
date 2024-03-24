use leetcode_api::{
    glob_leetcode,
    leetcode::{question::qs_detail::Question, IdSlug},
    render::Render,
};
use miette::Result;
use pretty_assertions::assert_eq;

#[ignore = "manual"]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_all_pbs_works() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    glob_leetcode()
        .await
        .sync_problem_index()
        .await?;
    Ok(())
}

#[ignore = "manual"]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn new_get_index() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    glob_leetcode()
        .await
        .sync_index_topic()
        .await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_user_code_work() -> Result<()> {
    let a = glob_leetcode()
        .await
        .get_user_code(IdSlug::Id(108))
        .await?;

    assert!(!a.0.is_empty());
    assert_eq!(&a.1, "[-10,-3,0,5,9]\n[1,3]");

    Ok(())
}

#[ignore = "manual"]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_work() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    match glob_leetcode()
        .await
        .test_code(IdSlug::Id(2))
        .await
    {
        Ok((_, res)) => {
            dbg!(&res);
            println!(r##"(| res |) -> {} "##, res.to_md_str(false));
            res.render_with_mdcat();
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
        .submit_code(IdSlug::Id(1))
        .await
        .unwrap();
    println!(r##"(| res |) -> {} "##, res.to_md_str(false));
    res.render_with_mdcat();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_qs_detail_work() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let lcode = glob_leetcode().await;
    let question = lcode
        .get_qs_detail(
            IdSlug::Slug("find-smallest-common-element-in-all-rows".to_owned()),
            true,
        )
        .await?;
    // println!("{:#?}", question.meta_data);
    // println!("{:#?}", question.stats);
    // println!("{:#?}", question.env_info);
    // dbg!(&question);
    assert_eq!(
        &question.qs_slug.unwrap(),
        "find-smallest-common-element-in-all-rows"
    );
    assert_eq!(
        &question.example_testcases,
        "[[1,2,3,4,5],[2,4,5,8,10],[3,5,7,9,11],[1,3,5,7,9]]\n[[1,2,3],[2,3,4],[2,3,5]]"
    );
    assert_eq!(
        &question.sample_test_case,
        "[[1,2,3,4,5],[2,4,5,8,10],[3,5,7,9,11],[1,3,5,7,9]]"
    );
    assert_eq!(
        &question.translated_title.unwrap(),
        "找出所有行中最小公共元素"
    );
    assert_eq!(&question.question_id, "1143");
    assert_eq!(
        &question.question_title.unwrap(),
        "Find Smallest Common Element in All Rows"
    );
    assert_eq!(&question.title, "Find Smallest Common Element in All Rows");

    let question = lcode
        .get_qs_detail(IdSlug::Slug("two-sum".to_owned()), true)
        .await?;
    assert_eq!(&question.question_id, "1");
    assert_eq!(&question.translated_title.unwrap(), "两数之和");
    assert_eq!(&question.title, "Two Sum");
    assert_eq!(&question.qs_slug.unwrap(), "two-sum");
    assert_eq!(&question.question_title.unwrap(), "Two Sum");
    let res = question
        .code_snippets
        .unwrap()
        .iter()
        .find(|x| &x.lang_slug == "rust")
        .unwrap()
        .clone();
    assert_eq!(
        &res.code,
        "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n\n    \
         }\n}"
    );

    let question = lcode
        .get_qs_detail(IdSlug::Id(195), true)
        .await?;
    assert_eq!(&question.qs_slug.unwrap(), "tenth-line");
    assert_eq!(&question.question_id, "195");
    assert_eq!(&question.question_title.unwrap(), "Tenth Line");
    assert_eq!(&question.title, "Tenth Line");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_qs_detail_none() {
    let question = glob_leetcode()
        .await
        .get_qs_detail(IdSlug::Id(0), false)
        .await
        .unwrap();
    assert_eq!(question, Question::default());
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
