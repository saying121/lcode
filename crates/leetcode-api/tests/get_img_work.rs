use leetcode_api::{glob_leetcode, leetcode::IdSlug};
use miette::Result;
use scraper::{Html, Selector};

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_img_url() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    // let question = glob_leetcode()
    //     .get_qs_detail(IdSlug::Id(113), true)
    //     .await?;
    let question = glob_leetcode()
        .await
        .get_qs_detail(IdSlug::Id(1008), true)
        .await?;

    let html = question
        .translated_content
        .unwrap();

    let fragment = Html::parse_fragment(&html);
    let selector = Selector::parse("img").unwrap();

    for element in fragment.select(&selector) {
        println!(
            "{}",
            element
                .value()
                .attr("src")
                .unwrap()
        );
    }

    Ok(())
}
