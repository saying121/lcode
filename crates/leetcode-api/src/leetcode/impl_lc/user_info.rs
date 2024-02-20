use std::path::PathBuf;

use lcode_config::config::global::{G_CACHE_DIR, G_USER_CONFIG};
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
        leetcode_send::fetch,
        resps::{
            checkin::{CheckInData, TotalPoints},
            pass_qs::{PassData, Passdata},
            user_data::{GlobData, UserStatus},
        },
        LeetCode,
    },
    Json,
};

// some infos
impl LeetCode {
    /// download user avator image
    pub async fn dow_user_avator(&self, status: &UserStatus) -> PathBuf {
        let avatar_url = status
            .avatar
            .as_deref()
            .unwrap_or_default();
        let mut avatar_path = G_CACHE_DIR.clone();
        if let Ok(url) = Url::parse(avatar_url) {
            if let Some(url_path) = url.path_segments() {
                let last = url_path.last().unwrap_or("avator.png");
                avatar_path.push(last);
            }
        };

        if let Ok(respond) = reqwest::get(avatar_url).await {
            if !avatar_path.exists() {
                let mut avatar_file = BufWriter::new(
                    OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(&avatar_path)
                        .await
                        .expect("create avatar failed"),
                );
                let var = respond.bytes().await.unwrap_or_default();
                avatar_file.write_all(&var).await.ok();
                avatar_file.flush().await.ok();
            }
        }
        avatar_path
    }
    pub async fn pass_qs_status(&self, user_slug: &str) -> Result<PassData> {
        let json = pass_status_grql(user_slug);
        let pat: Passdata = fetch(
            &self.client,
            &G_USER_CONFIG.urls.graphql,
            Some(&json),
            self.headers.clone(),
        )
        .await?;
        Ok(pat.data)
    }
    pub async fn get_points(&self) -> Result<TotalPoints> {
        fetch(
            &self.client,
            &G_USER_CONFIG.urls.points,
            None,
            self.headers.clone(),
        )
        .await
    }
    pub async fn get_user_info(&self) -> Result<UserStatus> {
        let json = global_data_grql();

        let resp: GlobData = fetch(
            &self.client,
            &G_USER_CONFIG.urls.graphql,
            Some(&json),
            self.headers.clone(),
        )
        .await?;

        Ok(resp.data.user_status)
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
        let json: Json = daily_checkin_grql();

        let (header_cn, header_com) = join!(
            Headers::build("leetcode.cn"),
            Headers::build("leetcode.com")
        );
        let (header_cn, header_com) = (
            header_cn.unwrap_or_default(),
            header_com.unwrap_or_default(),
        );

        let resp_cn = fetch::<CheckInData>(
            &self.client,
            "https://leetcode.cn/graphql",
            Some(&json),
            header_cn.headers,
        );

        let resp_com = fetch::<CheckInData>(
            &self.client,
            "https://leetcode.com/graphql",
            Some(&json),
            header_com.headers,
        );
        let (resp_cn, resp_com) = join!(resp_cn, resp_com);

        (resp_cn.unwrap_or_default(), resp_com.unwrap_or_default())
    }
}
