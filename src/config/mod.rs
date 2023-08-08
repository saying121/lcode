pub mod global;
pub mod read_config;
mod user_nest;

use self::global::global_user_config;
use crate::entities::prelude::*;
use miette::{miette, Error, IntoDiagnostic, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, path::PathBuf, str::FromStr};
use tokio::{fs::create_dir_all, join, task::spawn_blocking};
use tracing::{debug, trace};
use user_nest::*;

// get database connection
pub async fn conn_db() -> Result<DatabaseConnection, Error> {
    let db_dir = global::init_database_dir();
    create_dir_all(
        db_dir
            .parent()
            .unwrap_or_else(|| global::init_code_dir()),
    )
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

/// config for user
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub tongue: String,
    pub column: usize,
    pub urls: Urls,
    pub page_size: usize,
    support_lang: SupportLang,
    pub editor: VecDeque<String>,
    pub lang: String,
    pub code_dir: PathBuf,
    pub cookies: user_nest::Cookies,
}

impl Default for User {
    fn default() -> Self {
        Self {
            tongue: "en".to_owned(),
            column: 4,
            page_size: 25,
            urls: Urls::default(),
            editor: VecDeque::from([global::get_editor().clone()]),
            lang: "rust".to_owned(),
            code_dir: global::init_code_dir().clone(),
            cookies: user_nest::Cookies::default(),
            support_lang: SupportLang::default(),
        }
    }
}

impl User {
    /// "Chinese" "cn" "English" "en"
    pub fn new(tongue: &str) -> Self {
        let suffix = match tongue {
            "Chinese" => "cn",
            "cn" => "cn",
            "English" => "com",
            "en" => "com",
            _ => "com",
        };
        Self {
            tongue: match tongue {
                "Chinese" => "cn".to_owned(),
                "cn" => "cn".to_owned(),
                "English" => "en".to_owned(),
                "en" => "en".to_owned(),
                _ => "en".to_owned(),
            },
            urls: Urls {
                origin: format!("https://leetcode.{}", suffix),
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
            },
            editor: VecDeque::from([global::get_editor().clone()]),
            lang: "rust".to_owned(),
            code_dir: global::init_code_dir().clone(),
            cookies: user_nest::Cookies::default(),
            support_lang: SupportLang::default(),
            ..Default::default()
        }
    }

    pub fn mod_all_pb_api(&self, category: &str) -> String {
        self.urls
            .all_problem_api
            .replace("$category", category)
    }

    pub fn mod_submit(&self, slug: &str) -> String {
        self.urls
            .submit
            .replace("$slug", slug)
    }

    pub fn mod_test(&self, slug: &str) -> String {
        self.urls
            .test
            .replace("$slug", slug)
    }

    pub fn mod_submissions(&self, id: &str) -> String {
        self.urls
            .submissions
            .replace("$id", id)
    }

    /// get code file suffix
    pub fn get_suffix(&self) -> &str {
        let sp_lang = global::init_support_lang();
        sp_lang
            .get(self.lang.as_str())
            .unwrap_or(&".rs")
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
        let user = spawn_blocking(|| global_user_config().to_owned())
            .await
            .into_diagnostic()?;
        let cookies = user.cookies;

        let cookie = cookies.to_string();

        let kv_vec: Vec<(&str, &str)> = vec![
            ("Cookie", &cookie),
            ("x-csrftoken", &cookies.csrf),
            ("x-requested-with", "XMLHttpRequest"),
            ("Origin", &user.urls.origin),
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
