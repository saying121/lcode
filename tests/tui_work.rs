use lcode::mytui;
use miette::Result;

#[tokio::test]
async fn mytui_work() -> Result<()> {
    mytui::run().await?;
    Ok(())
}
