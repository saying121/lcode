use lcode_config::global::*;

#[cfg(target_os = "linux")]
#[test]
fn glob_path() {
    let home = dirs::home_dir().unwrap();
    assert_eq!(*G_CONFIG_PATH, home.join(".config/lcode/config.toml"));
    assert_eq!(*G_LANGS_PATH, home.join(".config/lcode/langs.toml"));
    assert_eq!(*G_COOKIES_PATH, home.join(".config/lcode/cookies.toml"));
}

#[cfg(target_os = "macos")]
#[test]
fn macos_path() {
    use std::path::PathBuf;

    use lcode_config::config::global::G_APP_NAME;
    // let a = init_config_path();
    let mut config_dir = dirs::config_dir().expect("new config dir failed");
    if std::env::consts::OS == "macos" {
        let home = std::env::var("HOME").unwrap();
        config_dir = PathBuf::from(home);
        config_dir.push(".config/");
    }

    config_dir.push(format!("{}/config.toml", G_APP_NAME));

    if std::env::consts::OS == "macos" {
        assert_eq!(
            config_dir.to_str().unwrap(),
            format!(
                "{}/{}",
                dirs::home_dir()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                ".config/lcode/config.toml"
            )
        );
    }
}
#[ignore = "Labor compare"]
#[test]
fn get_conf_work() {
    // _ = &USER_CONFIG.config;
    dbg!(&G_USER_CONFIG.keymap.keymap);
}
