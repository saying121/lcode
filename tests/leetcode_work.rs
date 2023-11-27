use lcode::{
    config::global::glob_leetcode,
    leetcode::{qs_detail::Question, IdSlug},
    render::*,
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
        .new_sync_index()
        .await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_user_code_work() -> Result<()> {
    let a = glob_leetcode()
        .get_user_code(IdSlug::Id(108))
        .await?;

    assert!(!a.0.is_empty());
    assert_eq!(&a.1, "[-10,-3,0,5,9]\n[1,3]");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_work() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    if let Ok((_, res)) = glob_leetcode()
        .test_code(IdSlug::Id(235))
        .await
    {
        println!(r##"(| res |) -> {} "##, res);
        render_str(&res.to_string())?;
    }

    Ok(())
}

#[ignore = "need realy environment"]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn submit_work() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let (_, res) = glob_leetcode()
        .submit_code(IdSlug::Id(45))
        .await?;
    println!(r##"(| res |) -> {} "##, res);
    render_str(&res.to_string())?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_qs_detail_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let lcode = glob_leetcode();
    let question = lcode
        .get_qs_detail(IdSlug::Id(1143), false)
        .await?;
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
        .get_qs_detail(IdSlug::Slug("two-sum".to_owned()), false)
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
    assert_eq!(&res.code, "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n\n    }\n}" );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_qs_detail_none() {
    let question = glob_leetcode()
        .get_qs_detail(IdSlug::Id(0), false)
        .await
        .unwrap();
    assert_eq!(question, Question::default());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_submit_list() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let a = glob_leetcode();
    let res = a
        .all_submit_res(IdSlug::Id(32))
        .await?;
    println!("{}", res);
    // render_str(res.to_string())?;
    // let res = get_rendered_str(res.to_string(), 30, 10)?;
    // println!("{}", res);

    Ok(())
}
