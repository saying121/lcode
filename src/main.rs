use lcode::cli::*;
use miette::Result;

#[tokio::main]
async fn main() -> Result<()> {

    run().await?;

    Ok(())
}
