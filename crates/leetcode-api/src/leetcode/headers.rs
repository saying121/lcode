use std::str::FromStr;
use strum::IntoEnumIterator;

use decrypt_cookies::{get_cookie, Browser};
use lcode_config::config::global::G_USER_CONFIG;
use miette::{IntoDiagnostic, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

/// headers for `LeetCode` reqwest
///
/// * `headers`: headers for reqwest
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub struct Headers {
    pub headers: HeaderMap,
}

impl Headers {
    pub async fn build_default() -> Result<Self> {
        let host = format!("{}.{}", "leetcode", G_USER_CONFIG.config.url_suffix);
        Self::build(&host).await
    }
    pub async fn build(host: &str) -> Result<Self> {
        let default_headers = HeaderMap::new();
        let mut cookies = G_USER_CONFIG.cookies.clone();

        if !cookies.is_completion() {
            let browser =
                Browser::from_str(G_USER_CONFIG.config.browser.as_str()).into_diagnostic()?;
            cookies = get_cookie(browser, host).await?;
        }

        if !cookies.is_completion() {
            for browser in Browser::iter() {
                let pat = get_cookie(browser, host)
                    .await
                    .unwrap_or_default();
                if pat.is_completion() {
                    cookies = pat;
                    break;
                }
            }
        }

        let cookie = cookies.to_string();

        let kv_vec: Vec<(&str, &str)> = vec![
            ("cookie", &cookie),
            ("x-csrftoken", &cookies.csrf),
            ("origin", &G_USER_CONFIG.urls.origin),
        ];
        let default_headers = Self::mod_headers(default_headers, kv_vec)?;

        Ok(Self { headers: default_headers })
    }

    /// new or modify headers
    ///
    /// * `headers`: be modified headers
    /// * `kv_vec`: added content
    pub fn mod_headers(mut headers: HeaderMap, kv_vec: Vec<(&str, &str)>) -> Result<HeaderMap> {
        for (k, v) in kv_vec {
            let name = HeaderName::from_str(k).into_diagnostic()?;
            let value = HeaderValue::from_str(v).into_diagnostic()?;

            headers.insert(name, value);
        }
        Ok(headers)
    }
}
