use std::{self, collections::HashMap, path::PathBuf, sync::OnceLock};

use super::{read_config::get_user_conf, User};

pub const APP_NAME: &str = "leetcode-cn-en-cli";

pub static USER_CONFIG: OnceLock<User> = OnceLock::new();
// global user config
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

pub const CATEGORIES: [&str; 4] = ["algorithms", "concurrency", "database", "shell"];

pub static DATABASE_DIR: OnceLock<PathBuf> = OnceLock::new();
/// "~/.cache/leetcode-cn-en-cli/leetcode.db"
pub fn init_database_dir() -> &'static PathBuf {
    DATABASE_DIR.get_or_init(|| {
        let mut db_dir = dirs::cache_dir().unwrap();
        db_dir.push(format!("{}/leetcode.db", APP_NAME));
        db_dir
    })
}

pub static CONF_PATH: OnceLock<PathBuf> = OnceLock::new();
// "/home/$USER/.cache/leetcode-cn-en-cli/problems/"
/// # Initialize the config directory
/// "~/.config/leetcode-cn-en-cli/config.toml"
pub fn init_config_path() -> &'static PathBuf {
    CONF_PATH.get_or_init(|| {
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push(format!("{}/config.toml", APP_NAME));
        config_dir
    })
}

pub static CODE_PATH: OnceLock<PathBuf> = OnceLock::new();
// "/home/$USER/.cache/leetcode-cn-en-cli/problems/"
/// # Initialize the config directory
/// "~/.local/share/leetcode-cn-en-cli"
pub fn init_code_dir() -> &'static PathBuf {
    CODE_PATH.get_or_init(|| {
        let mut code_dir = dirs::data_local_dir().unwrap();
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
