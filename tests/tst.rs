use lcode::editor::{self, edit};
use lcode::leetcode::IdSlug;
use miette::Result;

#[tokio::test]
async fn query_question_work() -> Result<()> {
    use lcode::{dao, leetcode::IdSlug};
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let a = dao::get_question_index(IdSlug::Id(0)).await?;
    println!(r##"(| 0 a |) -> {:#?}"##, a);
    let a = lcode::dao::get_question_index_exact(&IdSlug::Id(1)).await?;
    println!(r##"(| a |) -> {:#?}"##, a);

    Ok(())
}

#[tokio::test]
async fn edit_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    edit(IdSlug::Id(1_000_570), editor::CodeTestFile::Code).await?;
    edit(IdSlug::Id(1_000_570), editor::CodeTestFile::Test).await?;

    Ok(())
}
