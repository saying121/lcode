use lcode::{
    config::global::global_leetcode,
    leetcode::IdSlug,
    render::{self, Render},
};
use miette::Error;

#[tokio::test]
async fn render_html() -> Result<(), Error> {
    let a = global_leetcode();
    let qs = a.get_problem_detail(IdSlug::Id(1), false).await?;

    println!("{:#?}", qs.to_tui_vec());

    Ok(())
}

#[tokio::test]
async fn render_md_terminal() -> Result<(), Error> {
    let a = global_leetcode();
    let id = 1;
    let qs = a.get_problem_detail(IdSlug::Id(id), false).await?;

    use lcode::render::*;
    render_qs_to_tty(qs)?;

    Ok(())
}

#[tokio::test]
async fn render_md_str() -> Result<(), Error> {
    let a = global_leetcode();
    let id = 100092;
    println!("1");
    let qs = a.get_problem_detail(IdSlug::Id(id), false).await?;
    println!(r##"(| qs |) -> {:#?}"##, qs);

    use render::Render;
    let a = qs.to_rendered_str(80, 80)?;
    println!("{}", a);

    Ok(())
}
