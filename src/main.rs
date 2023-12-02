use lcode::cli::run;
use lcode::panic_hook::init_panic_hook;
use miette::Result;

#[tokio::main]
async fn main() -> Result<()> {
    init_panic_hook();

    run().await?;

    Ok(())
}
