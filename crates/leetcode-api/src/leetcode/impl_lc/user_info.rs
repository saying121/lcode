use std::path::PathBuf;

use lcode_config::global::{G_CACHE_DIR, G_USER_CONFIG};
use miette::Result;
use reqwest::Url;
use tokio::{
    fs::OpenOptions,
    io::{AsyncWriteExt, BufWriter},
    join,
};

use crate::{
    leetcode::{
        graphqls::*,
        headers::Headers,
        resps::{
            checkin::{CheckInData, TotalPoints},
            pass_qs::{PassData, Passdata},
            user_data::{GlobData, UserStatus},
        },
        LeetCode,
    },
    Json,
};

// some info
impl LeetCode {
    /// download user avatar image
    pub async fn dow_user_avator(&self, status: &UserStatus) -> Option<PathBuf> {
        let avatar_url = status.avatar.as_deref()?;
        let mut avatar_path = G_CACHE_DIR.clone();
        if let Ok(url) = Url::parse(avatar_url) {
            if let Some(url_path) = url.path_segments() {
                let last = url_path.last().unwrap_or("avator.jpeg");
                avatar_path.push(last);
            }
        };

        if let Ok(respond) = reqwest::get(avatar_url).await {
            if !avatar_path.exists() {
                if let Ok(f) = OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(&avatar_path)
                    .await
                {
                    let mut avatar_file = BufWriter::new(f);
                    let var = respond
                        .bytes()
                        .await
                        .unwrap_or_default();
                    avatar_file.write_all(&var).await.ok();
                    avatar_file.flush().await.ok();
                }
            }
        }
        Some(avatar_path)
    }
    pub async fn pass_qs_status(&self, user_slug: &str) -> Result<PassData> {
        let json = GraphqlQuery::pass_status(user_slug);
        let pat: Passdata = self
            .request(
                &G_USER_CONFIG.urls.graphql,
                Some(&json),
                self.headers.clone(),
            )
            .await?;
        Ok(pat.data)
    }
    pub async fn get_points(&self) -> Result<TotalPoints> {
        self.request(&G_USER_CONFIG.urls.points, None, self.headers.clone())
            .await
    }
    pub async fn get_user_info(&self) -> Result<UserStatus> {
        let json = GraphqlQuery::global_data();

        let resp: GlobData = self
            .request(
                &G_USER_CONFIG.urls.graphql,
                Some(&json),
                self.headers.clone(),
            )
            .await?;

        Ok(resp.user_status())
    }
    /// # Ensure that the cookies are obtained
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let status = glob_leetcode()
    ///     .await
    ///     .get_user_info()?
    ///     .unwrap();
    /// // if user_slug is None, the cookies were not obtained
    /// if !status.checked_in_today && status.user_slug.is_some() {
    ///     let res = glob_leetcode()
    ///         .await
    ///         .daily_checkin()
    ///         .await;
    /// }
    /// ```
    /// return order (cn, com)
    pub async fn daily_checkin(&self) -> (CheckInData, CheckInData) {
        let json: Json = GraphqlQuery::daily_checkin();

        let (header_cn, header_com) = join!(
            Headers::build("leetcode.cn"),
            Headers::build("leetcode.com")
        );
        let (header_cn, header_com) = (
            header_cn.unwrap_or_default(),
            header_com.unwrap_or_default(),
        );

        let resp_cn = self.request::<CheckInData>(
            "https://leetcode.cn/graphql",
            Some(&json),
            header_cn.headers,
        );

        let resp_com = self.request::<CheckInData>(
            "https://leetcode.com/graphql",
            Some(&json),
            header_com.headers,
        );
        let (resp_cn, resp_com) = join!(resp_cn, resp_com);

        (resp_cn.unwrap_or_default(), resp_com.unwrap_or_default())
    }
}
