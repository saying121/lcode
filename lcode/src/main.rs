use lcode::{cli::run, config::global::glob_config, panic_hook::init_panic_hook};
use miette::Result;

fn main() -> Result<()> {
    init_panic_hook();
    // init config
    glob_config();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(5) // enough
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    runtime.block_on(run())
}
