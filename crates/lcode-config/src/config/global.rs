use std::{collections::HashMap, fs::create_dir_all, path::PathBuf, sync::LazyLock};

use super::{read_config::get_user_conf, User};

pub const G_APP_NAME: &str = "leetcode-cn-en-cli";

/// # Get dir path and create dir
///
/// ~/.cache/leetcode-cn-en-cli/
pub static G_CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut log_dir = dirs::cache_dir().expect("new cache dir failed");
    create_dir_all(&log_dir).expect("create cache dir failed");
    log_dir.push(G_APP_NAME);
    log_dir
});

/// global user config
pub static G_USER_CONFIG: LazyLock<User> =
    LazyLock::new(|| get_user_conf().expect("get G_USER_CONFIG falied"));

/// "~/.cache/leetcode-cn-en-cli/leetcode.db"
pub static G_DATABASE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut db_dir = G_CACHE_DIR.clone();
    db_dir.push(format!("leetcode-{}.db", G_USER_CONFIG.config.url_suffix));
    db_dir
});

/// # Initialize the config directory create dir if not exists
/// "~/.config/leetcode-cn-en-cli/"
static G_CONF_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    #[cfg(not(target_os = "macos"))]
    let mut config_dir = dirs::config_dir().expect("new config dir failed");

    #[cfg(target_os = "macos")]
    let mut config_dir = {
        let mut dir = PathBuf::from(std::env::var("HOME").expect("get $HOME failed"));
        dir.push(".config/");
        dir
    };

    config_dir.push(G_APP_NAME);
    create_dir_all(&config_dir).expect("G_CONF_DIR create_dir_all failed");
    config_dir
});

/// # get the config path
/// "~/.config/leetcode-cn-en-cli/config.toml"
pub static G_CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = G_CONF_DIR.clone();
    dir.push("config.toml");
    dir
});

/// # get the cookies config path
/// "~/.config/leetcode-cn-en-cli/cookies.toml"
pub static G_COOKIES_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = G_CONF_DIR.clone();
    dir.push("cookies.toml");
    dir
});

/// # get the lang config path
/// "~/.config/leetcode-cn-en-cli/langs.toml"
pub static G_LANGS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = G_CONF_DIR.clone();
    dir.push("langs.toml");
    dir
});

/// # get the keymap config path
/// "~/.config/leetcode-cn-en-cli/keymap.toml"
pub static G_KEYMAP_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = G_CONF_DIR.clone();
    dir.push("keymap.toml");
    dir
});

pub static G_SUPPORT_LANGS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
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
