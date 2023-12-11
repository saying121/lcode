use lcode::editor::{self, edit};
use lcode::leetcode::IdSlug;
use miette::Result;

#[ignore = "manual"]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn edit_work() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    edit(IdSlug::Id(1_000_570), editor::CodeTestFile::Code).await?;
    edit(IdSlug::Id(1_000_570), editor::CodeTestFile::Test).await?;

    Ok(())
}
