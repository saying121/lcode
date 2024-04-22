use lcode::editor::{self, Editor};
use leetcode_api::leetcode::IdSlug;
use miette::Result;

#[ignore = "manual"]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn edit_work() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    Editor::open(IdSlug::Id(1_000_570), editor::CodeTestFile::Code).await?;
    Editor::open(IdSlug::Id(1_000_570), editor::CodeTestFile::Test).await?;

    Ok(())
}
