use std::{self, collections::HashMap, path::PathBuf, sync::OnceLock, thread};

use tokio::runtime::Builder;

use crate::leetcode::LeetCode;

use super::{read_config::get_user_conf, User};

pub const APP_NAME: &str = "leetcode-cn-en-cli";

pub static LOG_DIR: OnceLock<PathBuf> = OnceLock::new();
/// ~/.cache/leetcode-cn-en-cli/
pub fn global_log_dir() -> &'static PathBuf {
    LOG_DIR.get_or_init(|| {
        let mut log_dir = dirs::cache_dir().expect("new cache dir failed");
        log_dir.push(format!("{}", APP_NAME));
        log_dir
    })
}

pub static LEETCODE: OnceLock<LeetCode> = OnceLock::new();
/// global leetocde
pub fn global_leetcode() -> &'static LeetCode {
    LEETCODE.get_or_init(|| {
        thread::spawn(move || {
            let rt = Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("tokio runtime build failed");

            rt.block_on(async {
                LeetCode::new()
                    .await
                    .expect("new `LeetCode` failed")
            })
        })
        .join()
        .expect("generate leetcode failed")
    })
}

pub static USER_CONFIG: OnceLock<User> = OnceLock::new();
/// global user config
pub fn global_user_config() -> &'static User {
    USER_CONFIG.get_or_init(|| get_user_conf().unwrap_or_default())
}

pub static EDITOR: OnceLock<String> = OnceLock::new();
/// Get user's editor from environment variable EDITOR and VISUAL
pub fn get_editor() -> &'static String {
    EDITOR.get_or_init(|| match std::env::var("EDITOR") {
        Ok(v) => v,
        Err(_) => match std::env::var("VISUAL") {
            Ok(editor) => editor,
            Err(_) => "vim".to_string(),
        },
    })
}

pub const CATEGORIES: [&str; 6] = [
    "algorithms",
    "concurrency",
    "database",
    "shell",
    "javascript",
    "pandas",
];

pub static DATABASE_DIR: OnceLock<PathBuf> = OnceLock::new();
/// "~/.cache/leetcode-cn-en-cli/leetcode.db"
pub fn init_database_dir() -> &'static PathBuf {
    DATABASE_DIR.get_or_init(|| {
        let mut db_dir = dirs::cache_dir().expect("new cache dir failed");
        db_dir.push(format!("{}/leetcode.db", APP_NAME));
        db_dir
    })
}

pub static CONF_PATH: OnceLock<PathBuf> = OnceLock::new();
/// # Initialize the config directory
/// "~/.config/leetcode-cn-en-cli/config.toml"
pub fn init_config_path() -> &'static PathBuf {
    CONF_PATH.get_or_init(|| {
        let mut config_dir = dirs::config_dir().expect("new config dir failed");
        if std::env::consts::OS == "macos" {
            let home = std::env::var("HOME").unwrap();
            config_dir = PathBuf::from(home);
            config_dir.push(".config/")
        }

        config_dir.push(format!("{}/config.toml", APP_NAME));
        config_dir
    })
}

pub static CODE_PATH: OnceLock<PathBuf> = OnceLock::new();
/// # Initialize the config directory
/// "~/.local/share/leetcode-cn-en-cli"
pub fn init_code_dir() -> &'static PathBuf {
    CODE_PATH.get_or_init(|| {
        let mut code_dir = dirs::data_local_dir().expect("new data local dir failed");
        code_dir.push(APP_NAME);
        code_dir
    })
}

pub static SUPPORT_LANGS: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
pub fn init_support_lang() -> &'static HashMap<&'static str, &'static str> {
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
