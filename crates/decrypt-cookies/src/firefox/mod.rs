pub mod dao;
pub mod entities;

use std::path::PathBuf;

use ini::configparser::ini::Ini;
use miette::Result;
use tracing::debug;

use self::{dao::query_cookie, entities::moz_cookies};
use crate::{Browser, Cookies};

#[cfg(target_os = "linux")]
fn linux_path(browser: Browser) -> (PathBuf, &'static str) {
    const FIREFOX_LINUX: &str = ".mozilla/firefox";
    const LIBREWOLF_LINUX: &str = ".librewolf";

    let mut home = dirs::home_dir().expect("get home dir failed");
    let temp = match browser {
        Browser::Firefox => FIREFOX_LINUX,
        Browser::Librewolf => LIBREWOLF_LINUX,
        _ => FIREFOX_LINUX,
    };
    home.push(format!("{}/profiles.ini", temp));
    (home, temp)
}

#[cfg(target_os = "macos")]
fn macos_path(browser: Browser) -> (PathBuf, &'static str) {
    const FIREFOX_MAC: &str = "Library/Application Support/Firefox";
    const LIBREWOLF_MAC: &str = "Library/Application Support/librewolf";

    let mut home = dirs::home_dir().expect("get home dir failed");
    let temp = match browser {
        Browser::Firefox => FIREFOX_MAC,
        Browser::Librewolf => LIBREWOLF_MAC,
        _ => FIREFOX_MAC,
    };
    home.push(format!("{}/profiles.ini", temp));
    (home, temp)
}

#[cfg(target_os = "windows")]
fn win_path(browser: Browser) -> (PathBuf, &'static str) {
    const FIREFOX_WIN: &str = r"Mozilla\Firefox";
    const LIBREWOLF_WIN: &str = "librewolf";

    let mut home = dirs::home_dir().expect("get home dir failed");
    let temp = match browser {
        Browser::Firefox => FIREFOX_WIN,
        Browser::Librewolf => LIBREWOLF_WIN,
        _ => FIREFOX_WIN,
    };
    home.push(format!("{}/profiles.ini", temp));
    (home, temp)
}

fn get_cookie_path(browser: Browser) -> PathBuf {
    #[cfg(target_os = "linux")]
    let (bs, browser) = linux_path(browser);

    #[cfg(target_os = "macos")]
    let (bs, browser) = macos_path(browser);

    #[cfg(target_os = "windows")]
    let (bs, browser) = win_path(browser);

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
