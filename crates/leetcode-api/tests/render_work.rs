use leetcode_api::{glob_leetcode, leetcode::IdSlug, render::*};
use miette::Result;
use pretty_assertions::assert_eq;

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn content_img() -> Result<()> {
    let id = 1008;
    let qs = glob_leetcode()
        .await
        .get_qs_detail(IdSlug::Id(id), true)
        .await?;
    println!(
        "{}",
        qs.translated_content
            .as_deref()
            .unwrap()
    );
    let slug = "cnHoX6".to_owned();
    qs.render_with_mdcat();
    let qs = glob_leetcode()
        .await
        .get_qs_detail(IdSlug::Slug(slug), true)
        .await?;
    println!(
        "{}",
        qs.translated_content
            .as_deref()
            .unwrap()
    );
    qs.render_with_mdcat();
    Ok(())
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn render_md_terminal() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let id = 108;
    let qs = glob_leetcode()
        .await
        .get_qs_detail(IdSlug::Id(id), true)
        .await?;
    let slug = "ryfUiz".to_owned();
    qs.render_with_mdcat();
    let qs = glob_leetcode()
        .await
        .get_qs_detail(IdSlug::Slug(slug), true)
        .await?;
    // qs.render_to_terminal();
    dbg!(qs.to_tui_vec());

    Ok(())
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn render_md_str() -> Result<()> {
    let id = 654;
    let qs = glob_leetcode()
        .await
        .get_qs_detail(IdSlug::Id(id), false)
        .await?;

    let a = qs.to_md_str(true);
    println!("{}", a);

    Ok(())
}

#[test]
fn sub() {
    assert_eq!(&superscript(1_234_567_890, SupSub::Sup), "¹²³⁴⁵⁶⁷⁸⁹⁰");
    assert_eq!(&superscript(1_234_567_890, SupSub::Sub), "₁₂₃₄₅₆₇₈₉₀");
    assert_eq!(
        &to_sub_sup_script("<sup>123</sup>, <sub>456</sub>"),
        "¹²³, ₄₅₆"
    );
}
