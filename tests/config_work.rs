use std::path::PathBuf;

use lcode::config::global::{global_user_config, APP_NAME};

use miette::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt, prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt, Registry,
};

#[test]
fn macos_path() {
    // let a = init_config_path();
    let mut config_dir = dirs::config_dir().expect("new config dir failed");
    if std::env::consts::OS == "macos" {
        let home = std::env::var("HOME").unwrap();
        config_dir = PathBuf::from(home);
        config_dir.push(".config/")
    }

    config_dir.push(format!("{}/config.toml", APP_NAME));
    println!(r##"(| config_dir |) -> {:#?}"##, config_dir);
}

#[test]
fn get_conf_work() -> Result<()> {
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

    use lcode::config::read_config;
    let _a = read_config::gen_default_conf("cn")?;
    // let a = read_config::get_user_conf()?;
    // println!(r##"(| a |) -> {:#?}"##, a);
    let a = global_user_config();
    println!(r##"(| a |) -> {:#?}"##, a);


    Ok(())
}
