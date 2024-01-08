use lcode::mytui;
use miette::Result;

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn tui_run() -> Result<()> {
    Box::pin(mytui::run()).await?;
    let a = 1;
    print!("{a}");
    Ok(())
}
