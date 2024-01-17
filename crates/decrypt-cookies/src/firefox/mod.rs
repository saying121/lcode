pub mod dao;
pub mod entities;
mod path;

use std::path::PathBuf;

use ini::configparser::ini::Ini;
use miette::Result;
use tracing::debug;

use self::{dao::query_cookie, entities::moz_cookies};
use crate::{Browser, Cookies};

fn get_cookie_path(browser: Browser) -> PathBuf {
    #[cfg(target_os = "linux")]
    let (bs, browser) = path::linux_path(browser);

    #[cfg(target_os = "macos")]
    let (bs, browser) = path::macos_path(browser);

    #[cfg(target_os = "windows")]
    let (bs, browser) = path::win_path(browser);

    let mut ini_file = Ini::new();
    ini_file
        .load(bs.to_str().unwrap_or_default())
        .unwrap_or_default();

    let mut section = String::new();

    if let Some(map) = ini_file.get_map() {
        for (sect, val) in map {
            if sect.starts_with("install") {
                for (sect, val) in val {
                    if sect == "default" {
                        section = val.unwrap_or_default();
                    }
                }
            }
        }
    }
    debug!("section: {}", section);

    let mut cookies_db = dirs::home_dir().expect("get home dir failed");
    cookies_db.push(format!("{}/{}/cookies.sqlite", browser, section));

    cookies_db
}
pub async fn get_session_csrf(borwser: Browser, host: &str) -> Result<Cookies> {
    let cookies = query_cookie(borwser, host).await?;

    let mut res = Cookies::default();

    for cookie in cookies {
        if let Some(s) = cookie.name {
            if s == "csrftoken" {
                res.csrf = cookie.value.unwrap_or_default();
            }
            else if s == "LEETCODE_SESSION" {
                res.session = cookie.value.unwrap_or_default();
            }
        }
    }
    Ok(res)
}
pub async fn get_all_cookies(borwser: Browser, host: &str) -> Result<Vec<moz_cookies::Model>> {
    query_cookie(borwser, host).await
}
