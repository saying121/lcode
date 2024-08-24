mod graphqls;
pub mod headers;
pub mod impl_lc;
pub mod question;
pub mod resps;

use std::{fmt::Display, sync::atomic::AtomicU32, time::Duration};

use miette::{miette, Context, IntoDiagnostic, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use serde::de::DeserializeOwned;
use tracing::{debug, trace};

use self::headers::Headers;
use crate::Json;

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
    pub client: Client,
    pub headers: HeaderMap,
}

impl LeetCode {
    /// Create a `LeetCode` instance and initialize some variables
    pub async fn build() -> Result<Self> {
        let client = ClientBuilder::new()
            .brotli(true)
            .connect_timeout(Duration::from_secs(30))
            .build()
            .into_diagnostic()
            .context("reqwest client failed")?;

        Ok(Self {
            client,
            headers: Headers::build_default()
                .await
                .context("build header failed")?
                .headers,
        })
    }

    pub(super) async fn request<T>(
        &self,
        url: &str,
        json: Option<&Json>,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let headers = Headers::mod_headers(headers, vec![("Referer", url)])?;

        let req_builder = json.map_or_else(
            || self.client.get(url),
            |json| self.client.post(url).json(json),
        );

        let resp = req_builder
            .headers(headers)
            .send()
            .await
            .into_diagnostic()?;
        trace!("respond: {:#?}", resp);
        debug!("http status code: {:#?}", resp.status());

        match resp.status().as_u16() {
            403 => miette::bail!("Forbidden, maybe you not verify email or phone number"),
            408 => miette::bail!("Request Time-out"),
            429 => miette::bail!("Your submissions are too frequent."),
            400..500 => miette::bail!("Client error, HTTP Code: {}", resp.status()),
            500..600 => miette::bail!("Server error, HTTP Code: {}", resp.status()),
            _ => {},
        }

        resp.json::<T>()
            .await
            .map_err(|e| miette!("Error: {e}."))
    }
}
