use lcode::{fuzzy_search::select_a_question, glob_leetcode};
use leetcode_api::{dao::query::Query, leetcode::IdSlug, render::Render};
use miette::Result;
use unicode_width::UnicodeWidthStr;

#[ignore = "don't need"]
#[test]
fn width() {
    let a = "剑指 Offer 32 - III";
    let b = "a";
    println!("{:17}|", a);
    println!("{:19}|", b);

    let a = "剑指 Offer 10- II";
    let wd = UnicodeWidthStr::width(a);
    assert_eq!(wd, 17);
    let a = "II";
    let wd = UnicodeWidthStr::width(a);
    assert_eq!(wd, 2);
}

#[ignore = "need interact"]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn select_work() -> Result<()> {
    let id = select_a_question().await?;
    if id == 0 {
        return Ok(());
    }
    println!("{}", id);

    let a = glob_leetcode().await;
    let qs = a
        .get_qs_detail(IdSlug::Id(id), false)
        .await?;
    qs.render_with_mdcat();
    Ok(())
}

#[ignore = "just display"]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn index_display_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let idx = Query::query_all_index().await?;
    println!("{:#?}", idx[1]);
    for i in idx.iter().take(5) {
        print!("{}", i);
    }
    let length = idx.len();
    println!("{}", idx[length - 1]);
    println!("{}", idx[length - 2]);
    println!("{}", idx[length - 3]);

    Ok(())
}
