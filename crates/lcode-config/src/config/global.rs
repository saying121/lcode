use std::{collections::HashMap, fs::create_dir_all, path::PathBuf, sync::LazyLock};

use super::{read_config::get_user_conf, User};

pub const APP_NAME: &str = "leetcode-cn-en-cli";

/// ~/.cache/leetcode-cn-en-cli/
pub static LOG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut log_dir = dirs::cache_dir().expect("new cache dir failed");
    log_dir.push(APP_NAME);
    log_dir
});

/// global user config
pub static USER_CONFIG: LazyLock<User> = LazyLock::new(|| get_user_conf().unwrap());

/// "~/.cache/leetcode-cn-en-cli/leetcode.db"
pub static DATABASE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut db_dir = dirs::cache_dir().expect("new cache dir failed");
    db_dir.push(format!(
        "{}/leetcode-{}.db",
        APP_NAME, USER_CONFIG.config.url_suffix
    ));
    db_dir
});

/// # Initialize the config directory
/// "~/.config/leetcode-cn-en-cli/"
static CONF_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    #[cfg(not(target_os = "macos"))]
    let mut config_dir = dirs::config_dir().expect("new config dir failed");

    #[cfg(target_os = "macos")]
    let mut config_dir = {
        let mut dir = PathBuf::from(std::env::var("HOME").unwrap());
        dir.push(".config/");
        dir
    };

    config_dir.push(APP_NAME);
    create_dir_all(&config_dir).unwrap();
    config_dir
});

/// # get the config path
/// "~/.config/leetcode-cn-en-cli/config.toml"
pub static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = CONF_DIR.clone();
    dir.push("config.toml");
    dir
});

/// # get the config path
/// "~/.config/leetcode-cn-en-cli/cookies.toml"
pub static COOKIES_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = CONF_DIR.clone();
    dir.push("cookies.toml");
    dir
});

/// # get the config path
/// "~/.config/leetcode-cn-en-cli/langs.toml"
pub static LANGS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = CONF_DIR.clone();
    dir.push("langs.toml");
    dir
});

/// # get the keymap path
/// "~/.config/leetcode-cn-en-cli/keymap.toml"
pub static KEYMAP_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = CONF_DIR.clone();
    dir.push("keymap.toml");
    dir
});

pub static SUPPORT_LANGS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("rust", ".rs"),
        ("bash", ".sh"),
        ("c", ".c"),
        ("cpp", ".cpp"),
        ("csharp", ".cs"),
        ("golang", ".go"),
        ("java", ".java"),
        ("javascript", ".js"),
        ("kotlin", ".kt"),
        ("mysql", ".sql"),
        ("php", ".php"),
        ("python", ".py"),
        ("python3", ".py"),
        ("ruby", ".rb"),
        ("scala", ".scala"),
        ("swift", ".swift"),
        ("typescript", ".ts"),
        ("racket", ".rkt"),
        ("erlang", ".erl"),
        ("elixir", ".x"),
        ("dart", ".dart"),
        ("react", ".jsx"),
        ("Postgresql", ".sql"),
        ("oraclesql", ".sql"),
        ("mysql", ".sql"),
        ("mssql", ".sql"),
    ])
});
