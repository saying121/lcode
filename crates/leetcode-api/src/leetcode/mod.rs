mod graphqls;
pub mod headers;
pub mod impl_lc;
pub mod question;
pub mod resps;

use std::{fmt::Display, sync::atomic::AtomicU32, time::Duration};

use miette::{IntoDiagnostic, Result};
use reqwest::{header::HeaderMap, Client, ClientBuilder};

use self::headers::Headers;

pub const CATEGORIES: [&str; 8] = [
    "algorithms",
    "concurrency",
    "database",
    "javascript",
    "lcci",
    "lcof",
    "pandas",
    "shell",
];

/// for progress bar, total insert num
pub static TOTAL_QS_INDEX_NUM: AtomicU32 = AtomicU32::new(0);
/// for progress bar, current inserted num
pub static CUR_QS_INDEX_NUM: AtomicU32 = AtomicU32::new(0);

/// for progress bar, total insert num
pub static TOTAL_TOPIC_QS_INDEX_NUM: AtomicU32 = AtomicU32::new(0);
/// for progress bar, current inserted num
pub static CUR_TOPIC_QS_INDEX_NUM: AtomicU32 = AtomicU32::new(0);

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum IdSlug {
    Id(u32),
    Slug(String),
}

impl Display for IdSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(num) => num.fmt(f),
            Self::Slug(slug) => slug.fmt(f),
        }
    }
}

/// interact with leetcode.com/cn
#[derive(Default)]
#[derive(Debug)]
#[derive(Clone)]
pub struct LeetCode {
    pub client:  Client,
    pub headers: HeaderMap,
}

impl LeetCode {
    /// Create a `LeetCode` instance and initialize some variables
    pub async fn build() -> Result<Self> {
        let client = ClientBuilder::new()
            .gzip(true)
            .connect_timeout(Duration::from_secs(30))
            .build()
            .into_diagnostic()?;

        Ok(Self {
            client,
            headers: Headers::build_default().await?.headers,
        })
    }
}

mod leetcode_send {
    use miette::{miette, IntoDiagnostic, Result};
    use reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    };
    use serde::de::DeserializeOwned;
    use tracing::trace;

    use crate::{leetcode::headers::Headers, Json};

    pub(super) async fn fetch<T>(
        client: &Client,
        url: &str,
        json: Option<&Json>,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let headers = Headers::mod_headers(headers, vec![("Referer", url)])?;

        let temp = json.map_or_else(|| client.get(url), |json| client.post(url).json(json));

        let resp = temp
            .headers(headers)
            .send()
            .await
            .into_diagnostic()?;
        trace!("respond: {:#?}", resp);

        resp.json::<T>()
            .await
            .map_err(|e| miette!("Error: {e}."))
    }
}
