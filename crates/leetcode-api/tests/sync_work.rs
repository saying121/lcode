#![feature(duration_constructors)]
#![allow(clippy::unwrap_used)]

use std::time::Duration;

use leetcode_api::glob_leetcode;
use miette::Result;

fn trigger() -> bool {
    use std::fs::File;

    let db_path = &*lcode_config::global::G_DATABASE_PATH;
    if let Ok(f) = File::open(db_path) {
        if let Ok(meta) = f.metadata() {
            let Ok(ctime) = meta.created()
            else {
                return true;
            };
            let Ok(mtime) = meta.modified()
            else {
                return true;
            };
            // need init it
            if ctime == mtime {
                return true;
            }
            if let Ok(mod_time) = meta.modified() {
                if let Ok(elapsed) = mod_time.elapsed() {
                    return elapsed > Duration::from_days(100);
                }
            }
        }
    }
    true
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_all_pbs_works() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    if trigger() {
        glob_leetcode()
            .await
            .sync_problem_index()
            .await?;
    }
    Ok(())
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn new_get_index() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    if trigger() {
        glob_leetcode()
            .await
            .sync_index_topic()
            .await?;
    }
    Ok(())
}
