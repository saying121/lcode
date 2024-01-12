use std::path::PathBuf;

use lcode_config::config::global::{APP_NAME, CONFIG_PATH, COOKIES_PATH, LANGS_PATH, USER_CONFIG};

#[test]
fn glob_path() {
    let pat = &CONFIG_PATH;
    let a = pat.exists();
    println!("{a}");
    // println!("{:?}", &*pat);
    dbg!(&CONFIG_PATH);
    dbg!(&LANGS_PATH);
    dbg!(&COOKIES_PATH);
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
fn get_conf_work() {
    // _ = &USER_CONFIG.config;
    dbg!(&USER_CONFIG.keymap.keymap);
}
