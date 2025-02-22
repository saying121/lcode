use std::str::FromStr;

use decrypt_cookies::{browser::cookies::LeetCodeCookies, prelude::*};
use lcode_config::global::G_USER_CONFIG;
use miette::{IntoDiagnostic, Result, bail};
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
            cookies = get_cookie_by_browser(&G_USER_CONFIG.config.browser, host)
                .await
                .unwrap_or_default();
        }

        if !cookies.is_completion() {
            cookies = get_cookie(host)
                .await
                .unwrap_or_else(|err| {
                    tracing::warn!("{err}");
                    LeetCodeCookies::default()
                });
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

async fn get_cookie_by_browser(browser: &str, host: &str) -> Result<LeetCodeCookies> {
    macro_rules! ff_ck {
        ($ff:ident) => {
            let Ok(builder) = FirefoxBuilder::<$ff>::new()
            else {
                bail!("failed")
            };
            let Ok(getter) = builder.build().await
            else {
                bail!("failed")
            };
            let Ok(ck) = getter.get_session_csrf(host).await
            else {
                bail!("failed")
            };
            if ck.is_completion() {
                return Ok(ck);
            }
            bail!("failed")
        };
    }
    macro_rules! ch_ck {
        ($chromium:ident) => {
            let Ok(getter) = ChromiumBuilder::<$chromium>::new()
                .build()
                .await
            else {
                bail!("failed")
            };
            let Ok(ck) = getter
                .get_cookies_session_csrf(host)
                .await
            else {
                bail!("failed")
            };
            if ck.is_completion() {
                return Ok(ck);
            }
            bail!("failed")
        };
    }

    match browser.to_lowercase().as_str() {
        "firefox" => {
            ff_ck!(Firefox);
        },
        "librewolf" => {
            ff_ck!(Librewolf);
        },
        "chrome" => {
            ch_ck!(Chrome);
        },
        "edge" => {
            ch_ck!(Edge);
        },
        "chromium" => {
            ch_ck!(Chromium);
        },
        "brave" => {
            ch_ck!(Brave);
        },
        "vivaldi" => {
            ch_ck!(Vivaldi);
        },
        "yandex" => {
            ch_ck!(Yandex);
        },
        "opera" => {
            ch_ck!(Opera);
        },
        #[cfg(not(target_os = "linux"))]
        "operagx" => {
            ch_ck!(OperaGX);
        },
        #[cfg(not(target_os = "linux"))]
        "coccoc" => {
            ch_ck!(CocCoc);
        },
        _ => {
            bail!("failed")
        },
    }
}

async fn get_cookie(host: &str) -> Result<LeetCodeCookies> {
    macro_rules! ffs {
        ($($ff:ident), *,) => {
            $(
                'outer: {
                    let Ok(builder) = FirefoxBuilder::<$ff>::new()
                    else {
                        break 'outer;
                    };
                    let Ok(getter) = builder.build().await
                    else {
                        break 'outer;
                    };
                    let Ok(ck) = getter.get_session_csrf(host).await
                    else {
                        break 'outer;
                    };
                    if ck.is_completion() {
                        return Ok(ck);
                    }
                }
            )*
        };
    }
    ffs!(Firefox, Librewolf,);

    macro_rules! chromiums {
        ($($chromium:ident), *,) => {
            $(
                'outer: {
                    let Ok(chromium) = ChromiumBuilder::<$chromium>::new()
                        .build()
                        .await
                    else {
                        break 'outer;
                    };
                    let Ok(ck) = chromium
                        .get_cookies_session_csrf(host)
                        .await
                    else {
                        break 'outer;
                    };
                    if ck.is_completion() {
                        return Ok(ck);
                    }
                }
            )*
        };
    }
    chromiums!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi, Opera,);
    #[cfg(not(target_os = "linux"))]
    chromiums!(OperaGX, CocCoc,);

    bail!("failed")
}
