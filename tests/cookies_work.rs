use lcode::cookies::{decrypt_cookies, get_session_csrf};

use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt, Registry,
};

#[tokio::test]
async fn get_cookie_work() -> miette::Result<()> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    let formatting_layer = fmt::layer()
        .pretty()
        .with_writer(std::io::stderr);
    Registry::default()
        .with(env_filter)
        .with(ErrorLayer::default())
        .with(formatting_layer)
        .init();

    let a = get_session_csrf().await?;
    // println!(r##"(| a |) -> {:?}"##, a);
    let a = decrypt_cookies(a.get("csrftoken").unwrap()).await?;
    println!("{}", a);
    // let a = decrypt_cookies(a.get("LEETCODE_SESSION").unwrap()).await?;
    // println!("{}", a);

    Ok(())
}
