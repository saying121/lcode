pub mod dao;
pub mod entities;

use std::path::PathBuf;

use ini::configparser::ini::Ini;
use miette::Result;
use tracing::debug;

use self::dao::query_cookie;
use crate::{Browser, Cookies};

pub const FIREFOX_LINUX: &str = ".mozilla/firefox";
pub const FIREFOX_WIN: &str = r"Mozilla\Firefox";
pub const FIREFOX_MAC: &str = "Library/Application Support/Firefox";
pub const LIBREWOLF_LINUX: &str = ".librewolf";
pub const LIBREWOLF_MAC: &str = "Library/Application Support/librewolf";
pub const LIBREWOLF_WIN: &str = "librewolf";

fn get_cookie_path(select: Browser) -> PathBuf {
    let home = dirs::home_dir().expect("get home dir failed");

    #[cfg(target_os = "linux")]
    let (bs, browser) = {
        let temp = match select {
            Browser::Firefox => FIREFOX_LINUX,
            Browser::Librewolf => LIBREWOLF_LINUX,
            _ => FIREFOX_LINUX,
        };
        let mut bs = home.clone();
        bs.push(format!("{}/profiles.ini", temp));
        (bs, temp)
    };
    #[cfg(target_os = "macos")]
    let (bs, browser) = {
        let temp = match select {
            Browser::Firefox => FIREFOX_MAC,
            Browser::Librewolf => LIBREWOLF_MAC,
            _ => FIREFOX_MAC,
        };
        let mut bs = home.to_owned();
        bs.push(format!("{}/profiles.ini", temp));
        (bs, temp)
    };

    #[cfg(target_os = "windows")]
    let (bs, browser) = {
        let temp = match select {
            Browser::Firefox => FIREFOX_WIN,
            Browser::Librewolf => LIBREWOLF_WIN,
            _ => FIREFOX_WIN,
        };
        let mut bs = home.to_owned();
        bs.push(format!("{}/profiles.ini", temp));
        (bs, temp)
    };

    let mut inif = Ini::new();
    inif.load(bs.to_str().unwrap_or_default())
        .unwrap_or_default();

    let mut section = String::new();

    for (sect, val) in inif.get_map().unwrap_or_default() {
        if sect.starts_with("install") {
            for (sect, val) in val {
                if sect == "default" {
                    section = val.unwrap_or_default();
                }
            }
        }
    }
    debug!("section: {}", section);

    let mut cookies_db = home;
    cookies_db.push(format!("{}/{}/cookies.sqlite", browser, section));

    cookies_db
}
pub async fn get_session_csrf(borwser: Browser, host: &str) -> Result<Cookies> {
    let cookies = query_cookie(borwser, host).await?;

    let mut res = Cookies::default();

    for cookie in cookies {
        if cookie
            .name
            .as_ref()
            .cloned()
            .unwrap_or_default()
            == "csrftoken"
        {
            res.csrf = cookie.value.unwrap_or_default();
        }
        else if cookie
            .name
            .as_ref()
            .cloned()
            .unwrap_or_default()
            == "LEETCODE_SESSION"
        {
            res.session = cookie.value.unwrap_or_default();
        }
    }
    Ok(res)
}
