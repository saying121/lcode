use lcode::{
    config::global::global_leetcode,
    leetcode::IdSlug,
    render::{self, pre_render, Render},
};
use miette::Result;

#[tokio::test]
async fn render_html() -> Result<()> {
    let a = global_leetcode();
    let qs = a
        .get_qs_detail(IdSlug::Id(1), false)
        .await?;

    println!("{:#?}", qs.to_tui_vec());

    Ok(())
}

#[tokio::test]
async fn render_md_terminal() -> Result<()> {
    let a = global_leetcode();
    let id = 1;
    let qs = a
        .get_qs_detail(IdSlug::Id(id), false)
        .await?;

    use lcode::render::*;
    render_qs_to_tty(qs)?;

    Ok(())
}

#[tokio::test]
async fn render_md_str() -> Result<()> {
    let a = global_leetcode();
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
    let a = global_leetcode();
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
    let a = global_leetcode();
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
