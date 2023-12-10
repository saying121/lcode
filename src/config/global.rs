use std::{collections::HashMap, fs::create_dir_all, path::PathBuf, sync::OnceLock};

use tokio::sync::OnceCell;

use crate::leetcode::LeetCode;

use super::{read_config::get_user_conf, User};

pub const CATEGORIES: [&str; 8] = [
    "algorithms",
    "concurrency",
    "database",
    "javascript",
    "lcci",
    "lcof",
    "pandas",
    "shell",
];

pub const APP_NAME: &str = "leetcode-cn-en-cli";

pub static LOG_DIR: OnceLock<PathBuf> = OnceLock::new();
/// ~/.cache/leetcode-cn-en-cli/
pub fn glob_log_dir() -> &'static PathBuf {
    LOG_DIR.get_or_init(|| {
        let mut log_dir = dirs::cache_dir().expect("new cache dir failed");
        log_dir.push(APP_NAME);
        log_dir
    })
}

pub static LEETCODE: OnceCell<LeetCode> = OnceCell::const_new();
/// global leetocde
pub async fn glob_leetcode() -> &'static LeetCode {
    LEETCODE
        .get_or_init(|| async {
            LeetCode::new()
                .await
                .expect("new `LeetCode` failed")
        })
        .await
}

pub static USER_CONFIG: OnceLock<User> = OnceLock::new();
/// global user config
pub fn glob_user_config() -> &'static User {
    USER_CONFIG.get_or_init(|| get_user_conf().unwrap())
}

pub static DATABASE_DIR: OnceLock<PathBuf> = OnceLock::new();
/// "~/.cache/leetcode-cn-en-cli/leetcode.db"
pub fn glob_database_path() -> &'static PathBuf {
    DATABASE_DIR.get_or_init(|| {
        let mut db_dir = dirs::cache_dir().expect("new cache dir failed");
        db_dir.push(format!("{}/leetcode.db", APP_NAME));
        db_dir
    })
}

pub static CONF_DIR: OnceLock<PathBuf> = OnceLock::new();
/// # Initialize the config directory
/// "~/.config/leetcode-cn-en-cli/"
pub fn glob_config_dir() -> &'static PathBuf {
    CONF_DIR.get_or_init(|| {
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
    })
}
pub static CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();
/// # get the config path
/// "~/.config/leetcode-cn-en-cli/config.toml"
pub fn glob_config_path() -> &'static PathBuf {
    CONFIG_PATH.get_or_init(|| {
        let mut dir = glob_config_dir().clone();
        dir.push("config.toml");
        dir
    })
}
pub static COOKIES_PATH: OnceLock<PathBuf> = OnceLock::new();
/// # get the config path
/// "~/.config/leetcode-cn-en-cli/cookies.toml"
pub fn glob_cookies_path() -> &'static PathBuf {
    COOKIES_PATH.get_or_init(|| {
        let mut dir = glob_config_dir().clone();
        dir.push("cookies.toml");
        dir
    })
}
pub static LANGS_PATH: OnceLock<PathBuf> = OnceLock::new();
/// # get the config path
/// "~/.config/leetcode-cn-en-cli/langs.toml"
pub fn glob_langs_path() -> &'static PathBuf {
    LANGS_PATH.get_or_init(|| {
        let mut dir = glob_config_dir().clone();
        dir.push("langs.toml");
        dir
    })
}

pub static SUPPORT_LANGS: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
pub fn glob_support_lang() -> &'static HashMap<&'static str, &'static str> {
    SUPPORT_LANGS.get_or_init(|| {
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
        ])
    })
}
