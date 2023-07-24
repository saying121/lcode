pub mod read_config;

use miette::{miette, Error};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr, sync::OnceLock};

pub const CATEGORIES: [&str; 4] =
    ["algorithms", "concurrency", "database", "shell"];

pub static DATABASE_DIR: OnceLock<PathBuf> = OnceLock::new();
/// "~/.cache/leetcode-cn-cli/leetcode.db"
pub fn init_database_dir() -> &'static PathBuf {
    DATABASE_DIR.get_or_init(|| {
        let mut a = dirs::cache_dir().unwrap();
        a.push("leetcode-cn-cli/leetcode.db");
        a
    })
}

pub static CACHE_PROBLEM_PATH: OnceLock<PathBuf> = OnceLock::new(); // "/home/$USER/.cache/leetcode-cn-cli/problems/"
/// Initialize the cache directory
/// "~/.cache/leetcode-cn-cli/problems/"
pub fn init_cache_dir() -> &'static PathBuf {
    CACHE_PROBLEM_PATH.get_or_init(|| {
        let mut a = dirs::cache_dir().unwrap();
        a.push("leetcode-cn-cli/problems");
        a
    })
}

pub static CACHE_PROBLEM_DETAIL_PATH: OnceLock<PathBuf> = OnceLock::new(); // "/home/$USER/.cache/leetcode-cn-cli/problems/"
/// "~/.cache/leetcode-cn-cli/problems/"
pub fn init_cache_detail_dir() -> &'static PathBuf {
    CACHE_PROBLEM_DETAIL_PATH.get_or_init(|| {
        let mut a = dirs::cache_dir().unwrap();
        a.push("leetcode-cn-cli/problem_details");
        a
    })
}
pub static CONF_PATH: OnceLock<PathBuf> = OnceLock::new(); // "/home/$USER/.cache/leetcode-cn-cli/problems/"
/// # Initialize the config directory
/// "~/.config/leetcode-cn-cli/config.toml"
pub fn init_config_dir() -> &'static PathBuf {
    CONF_PATH.get_or_init(|| {
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push("leetcode-cn-cli".to_string() + "/config.toml");
        config_dir
    })
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cookies {
    pub csrf: String,
    pub session: String,
}

impl ToString for Cookies {
    fn to_string(&self) -> String {
        format!("LEETCODE_SESSION={};csrftoken={};", self.session, self.csrf)
    }
}

/// config for user
///
/// * `base_url`: leetcode url
/// * `graphql`: leetcode graphql api url
/// * `all_problem_api`: leetcode api
/// * `cookie`: user's cookie
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub origin_url: String,
    pub graphql: String,
    pub all_problem_api: String,
    pub cookie: Cookies,
}

impl Default for User {
    fn default() -> Self {
        Self {
            origin_url: "https://leetcode.com".to_string(),
            graphql: "https://leetcode.com/graphql".to_string(),
            all_problem_api: "https://leetcode.com/api/problems/$category"
                .to_string(),
            cookie: Cookies::default(),
        }
    }
}

/// 开发者用的配置
///
/// * `headers`: reqwest 使用的 headers
pub struct Config {
    pub headers: HeaderMap,
}

impl Config {
    pub async fn new() -> Result<Self, Error> {
        let default_headers = HeaderMap::new();
        let user = read_config::get_user_conf().await?;
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

    /// new/modify headers
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
