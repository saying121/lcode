use lcode::{cli::run, panic_hook::init_panic_hook};
use lcode_config::config::global::USER_CONFIG;
use miette::Result;

fn main() -> Result<()> {
    init_panic_hook();
    // init config
    _ = &USER_CONFIG.config;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(5) // enough
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    runtime.block_on(run())
}
