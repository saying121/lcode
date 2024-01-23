use std::str::FromStr;

use decrypt_cookies::{get_cookie, Browser};
use lcode_config::config::global::G_USER_CONFIG;
use miette::{IntoDiagnostic, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

/// headers for `LeetCode` reqwest
///
/// * `headers`: headers for reqwest
#[derive(Default)]
pub struct Headers {
    pub headers: HeaderMap,
}
const BROWSERS: [Browser; 4] = [
    Browser::Firefox,
    Browser::Edge,
    Browser::Chrome,
    Browser::Librewolf,
];

impl Headers {
    pub async fn build_default() -> Result<Self> {
        let host = format!("{}.{}", "leetcode", G_USER_CONFIG.config.url_suffix);
        Self::build(&host).await
    }
    pub async fn build(host: &str) -> Result<Self> {
        let default_headers = HeaderMap::new();
        let mut cookies = G_USER_CONFIG.cookies.clone();

        if !cookies.is_completion() {
            cookies = get_cookie(G_USER_CONFIG.config.browser.as_str(), host).await?;
        }

        if !cookies.is_completion() {
            for i in BROWSERS {
                let pat = get_cookie(i, host)
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
