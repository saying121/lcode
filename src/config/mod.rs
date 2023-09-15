pub mod global;
pub mod read_config;
pub mod user_nest;

use std::{collections::VecDeque, path::PathBuf, str::FromStr};

use self::global::glob_user_config;
use miette::{Error, IntoDiagnostic, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;
use user_nest::*;

use crate::cookies::get_cookie;

/// config for user
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub translate: bool,
    #[serde(default)]
    pub column: usize,
    #[serde(default)]
    pub num_sublist: u32,
    #[serde(default)]
    pub url_suffix: String,
    #[serde(skip)]
    pub urls: Urls,
    #[serde(default)]
    pub page_size: usize,
    #[serde(default)]
    support_lang: SupportLang,
    #[serde(default)]
    pub editor: VecDeque<String>,
    #[serde(default)]
    pub lang: String,
    #[serde(default)]
    pub code_dir: PathBuf,
    #[serde(default)]
    pub browser: String,
    #[serde(default)]
    pub cookies: user_nest::Cookies,
}

impl Default for User {
    fn default() -> Self {
        Self {
            translate: false,
            column: 4,
            num_sublist: 10,
            page_size: 25,
            url_suffix: "com".to_owned(),
            urls: Urls::default(),
            editor: VecDeque::from([global::glob_editor().clone()]),
            lang: "rust".to_owned(),
            code_dir: global::glob_code_dir().clone(),
            browser: "".to_owned(),
            cookies: user_nest::Cookies::default(),
            support_lang: SupportLang::default(),
        }
    }
}

impl User {
    ///  "cn"  "en"
    pub fn new(tongue: &str) -> Self {
        let (suffix, translate) = match tongue {
            "cn" => ("cn", true),
            "en" => ("com", false),
            _ => ("com", false),
        };
        Self {
            translate,
            urls: Urls {
                origin: format!("https://leetcode.{}", suffix),
                graphql: format!("https://leetcode.{}/graphql", suffix),
                question_url: format!("https://leetcode.{}/problems/$slug/", suffix),
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
            editor: VecDeque::from([global::glob_editor().clone()]),
            lang: "rust".to_owned(),
            code_dir: global::glob_code_dir().clone(),
            cookies: user_nest::Cookies::default(),
            support_lang: SupportLang::default(),
            url_suffix: suffix.to_string(),
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
    pub fn get_qsurl(&self, slug: &str) -> String {
        self.urls
            .question_url
            .replace("$slug", slug)
    }

    /// get code file suffix
    pub fn get_suffix(&self) -> &str {
        let sp_lang = global::glob_support_lang();
        sp_lang
            .get(self.lang.as_str())
            .cloned()
            .unwrap_or_default()
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
        let user = spawn_blocking(glob_user_config)
            .await
            .into_diagnostic()?;
        let mut cookies = user.cookies.clone();
        if cookies.csrf.is_empty() || cookies.session.is_empty() {
            cookies = get_cookie(&user.browser)
                .await
                .unwrap_or_default();
        }

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
            let name = HeaderName::from_str(k).into_diagnostic()?;
            let value = HeaderValue::from_str(v).into_diagnostic()?;

            headers.insert(name, value);
        }
        Ok(headers)
    }
}
