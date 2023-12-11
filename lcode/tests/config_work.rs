use std::path::PathBuf;

use lcode::config::global::{glob_config, APP_NAME, glob_config_dir, glob_config_path, glob_langs_path, glob_cookies_path};

use miette::Result;

#[test]
fn glob_path() {
    dbg!(glob_config_dir());
    dbg!(glob_config_path());
    dbg!(glob_langs_path());
    dbg!(glob_cookies_path());
}

#[test]
fn macos_path() {
    // let a = init_config_path();
    let mut config_dir = dirs::config_dir().expect("new config dir failed");
    if std::env::consts::OS == "macos" {
        let home = std::env::var("HOME").unwrap();
        config_dir = PathBuf::from(home);
        config_dir.push(".config/");
    }

    config_dir.push(format!("{}/config.toml", APP_NAME));
    dbg!(&config_dir);

    if std::env::consts::OS == "macos" {
        assert_eq!(
            config_dir.to_str().unwrap(),
            format!(
                "{}/{}",
                dirs::home_dir()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                ".config/leetcode-cn-en-cli/config.toml"
            )
        );
    }
}
#[ignore = "Labor compare"]
#[test]
fn get_conf_work() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    use lcode::config::read_config;
    read_config::gen_default_conf(read_config::Tongue::Cn)?;
    // let a = read_config::get_user_conf()?;
    // println!(r##"(| a |) -> {:#?}"##, a);
    let a = glob_config();
    dbg!(a);

    Ok(())
}
