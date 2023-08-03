pub mod read_config;
mod user_nest;

use crate::entities::prelude::*;
use miette::{miette, Error, IntoDiagnostic, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr, sync::OnceLock};
use tokio::{fs::create_dir_all, join};
use tracing::{debug, trace};

use self::read_config::get_user_conf;

const APP_NAME: &str = "leetcode-cn-en-cli";
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

pub static CONF_PATH: OnceLock<PathBuf> = OnceLock::new(); // "/home/$USER/.cache/leetcode-cn-en-cli/problems/"
/// # Initialize the config directory
/// "~/.config/leetcode-cn-en-cli/config.toml"
pub fn init_config_path() -> &'static PathBuf {
    CONF_PATH.get_or_init(|| {
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push(format!("{}/config.toml", APP_NAME));
        config_dir
    })
}

pub static CODE_PATH: OnceLock<PathBuf> = OnceLock::new(); // "/home/$USER/.cache/leetcode-cn-en-cli/problems/"
/// # Initialize the config directory
/// "~/.local/share/leetcode-cn-en-cli"
pub fn init_code_dir() -> &'static PathBuf {
    CODE_PATH.get_or_init(|| {
        let mut code_dir = dirs::data_local_dir().unwrap();
        code_dir.push(APP_NAME);
        code_dir
    })
}

// get database connection
pub async fn conn_db() -> Result<DatabaseConnection, Error> {
    let db_dir = init_database_dir();
    create_dir_all(db_dir.parent().unwrap())
        .await
        .into_diagnostic()?;
    let db_conn_str = format!(
        "sqlite:{}?mode=rwc",
        db_dir
            .to_string_lossy()
            .to_string()
    );
    debug!("database dir: {}", &db_conn_str);

    let db = Database::connect(db_conn_str)
        .await
        .into_diagnostic()?;
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let stmt_index = builder.build(
        schema
            .create_table_from_entity(Index)
            .if_not_exists(),
    );
    let stmt_detail = builder.build(
        schema
            .create_table_from_entity(Detail)
            .if_not_exists(),
    );
    // new table
    let (index_res, detail_res) = join!(db.execute(stmt_index), db.execute(stmt_detail));
    let (index_exec, detail_exec) =
        (index_res.into_diagnostic()?, detail_res.into_diagnostic()?);
    trace!("create database: {:?},{:?}", index_exec, detail_exec);

    Ok(db)
}

use user_nest::*;
/// config for user
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub origin_url: String,
    pub graphql: String,
    pub all_problem_api: String,
    pub cookie: user_nest::Cookies,
    pub submit: String,
    pub test: String,
    pub submissions: String,
    pub favorites: String,
    pub editor: Vec<String>,
    pub lang: String,
    pub code_dir: PathBuf,
    support_lang: SupportLang,
}

impl Default for User {
    fn default() -> Self {
        let suffix ="com";

        Self {
            origin_url: format!("https://leetcode.{}", suffix),
            graphql: format!("https://leetcode.{}/graphql", suffix),
            all_problem_api: format!(
                "https://leetcode.{}/api/problems/$category",
                suffix
            ),
            submit: format!("https://leetcode.{}/problems/$slug/submit/", suffix),
            test: format!(
                "https://leetcode.{}/problems/$slug/interpret_solution/",
                suffix
            ),
            submissions: format!(
                "https://leetcode.{}/submissions/detail/$id/check/",
                suffix
            ),
            favorites: format!("https://leetcode.{}/list/api/questions", suffix),
            editor: vec!["vim".to_string()],
            lang: "rust".to_owned(),
            code_dir: init_code_dir().clone(),
            cookie: user_nest::Cookies::default(),
            support_lang: SupportLang::default(),
        }
    }
}

impl User {
    /// "Chinese" "cn" "English" "en"
    pub fn new(language: &str) -> Self {
        let suffix = match language {
            "Chinese" => "cn",
            "cn" => "cn",
            "English" => "com",
            "en" => "com",
            _ => "com",
        };

        Self {
            origin_url: format!("https://leetcode.{}", suffix),
            graphql: format!("https://leetcode.{}/graphql", suffix),
            all_problem_api: format!(
                "https://leetcode.{}/api/problems/$category",
                suffix
            ),
            submit: format!("https://leetcode.{}/problems/$slug/submit/", suffix),
            test: format!(
                "https://leetcode.{}/problems/$slug/interpret_solution/",
                suffix
            ),
            submissions: format!(
                "https://leetcode.{}/submissions/detail/$id/check/",
                suffix
            ),
            favorites: format!("https://leetcode.{}/list/api/questions", suffix),
            editor: vec!["vim".to_string()],
            lang: "rust".to_owned(),
            code_dir: init_code_dir().clone(),
            cookie: user_nest::Cookies::default(),
            support_lang: SupportLang::default(),
        }
    }

    pub fn mod_all_pb_api(&self, category: &str) -> String {
        self.all_problem_api
            .replace("$category", category)
    }

    pub fn mod_submit(&self, slug: &str) -> String {
        self.submit.replace("$slug", slug)
    }

    pub fn mod_test(&self, slug: &str) -> String {
        self.test.replace("$slug", slug)
    }

    pub fn mod_submissions(&self, id: &str) -> String {
        self.submissions.replace("$id", id)
    }

    /// get code file suffix
    pub fn get_suffix(&self) -> &str {
        match self.lang.as_str() {
            "rust" => ".rs",
            "bash" => ".sh",
            "c" => ".c",
            "cpp" => ".cpp",
            "csharp" => ".cs",
            "golang" => ".go",
            "java" => ".java",
            "javascript" => ".js",
            "kotlin" => ".kt",
            "mysql" => ".sql",
            "php" => ".php",
            "python" => ".py",
            "python3" => ".py",
            "ruby" => ".rb",
            "scala" => ".scala",
            "swift" => ".swift",
            "typescript" => ".ts",
            "racket" => ".rkt",
            "erlang" => ".erl",
            "elixir" => ".x",
            "dart" => ".dart",
            _ => "",
        }
    }
}

/// config for developer
///
/// * `headers`: headers for reqwest
pub struct Config {
    pub headers: HeaderMap,
}

impl Config {
    pub async fn new() -> Result<Self, Error> {
        let default_headers = HeaderMap::new();
        let user = get_user_conf().await?;
        let cookies = user.cookie;

        let cookie = cookies.to_string();

        let kv_vec: Vec<(&str, &str)> = vec![
            ("Cookie", &cookie),
            ("x-csrftoken", &cookies.csrf),
            ("x-requested-with", "XMLHttpRequest"),
            ("Origin", &user.origin_url),
        ];
        let default_headers = Self::mod_headers(default_headers, kv_vec)?;

        Ok(Config {
            headers: default_headers,
        })
    }

    /// new or modify headers
    ///
    /// * `headers`: be modified headers
    /// * `kv_vec`: added content
    pub fn mod_headers(
        mut headers: HeaderMap,
        kv_vec: Vec<(&str, &str)>,
    ) -> Result<HeaderMap, Error> {
        for (k, v) in kv_vec {
            let name = HeaderName::from_str(k);
            let value = HeaderValue::from_str(v);
            if name.is_err() || value.is_err() {
                return Err(miette!("headers modify error"));
            }

            headers.insert(name.unwrap(), value.unwrap());
        }
        Ok(headers)
    }
}
