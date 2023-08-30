use std::{collections::HashMap, path::PathBuf};

use dirs::{data_dir, home_dir};
use ini::configparser::ini::Ini;
use miette::{IntoDiagnostic, Result};
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::debug;

use crate::config::{global::global_user_config, user_nest::Cookies};
use crate::cookies::ff_base_entities::prelude::*;

use super::ff_base_entities::moz_cookies::{self, Model};

pub const FIREFOX_LINUX: &str = ".mozilla/firefox";
pub const FIREFOX_WIN: &str = r"Mozilla\Firefox";
pub const FIREFOX_MAC: &str = "Library/Application Support/Firefox";
pub const LIBREWOLF_LINUX: &str = ".librewolf";
pub const LIBREWOLF_MAC: &str = "Library/Application Support/librewolf";
pub const LIBREWOLF_WIN: &str = "librewolf";

async fn get_cookie_file(select: &str) -> Result<PathBuf> {
    let (bs, browser) = match std::env::consts::OS {
        "linux" => {
            let temp = match select {
                "firefox" => FIREFOX_LINUX,
                "librewolf" => LIBREWOLF_LINUX,
                _ => FIREFOX_LINUX,
            };
            let mut bs = home_dir().expect("get home dir failed");
            bs.push(format!("{}/profiles.ini", temp));
            (bs, temp)
        }
        "macos" => {
            let temp = match select {
                "firefox" => FIREFOX_MAC,
                "librewolf" => LIBREWOLF_MAC,
                _ => FIREFOX_MAC,
            };
            let mut bs = home_dir().expect("get home dir failed");
            bs.push(format!("{}/profiles.ini", temp));
            (bs, temp)
        }
        "windows" => {
            let temp = match select {
                "firefox" => FIREFOX_WIN,
                "librewolf" => LIBREWOLF_WIN,
                _ => FIREFOX_WIN,
            };
            let mut bs = data_dir().expect("get home dir failed");
            bs.push(format!("{}/profiles.ini", temp));
            (bs, temp)
        }
        _ => (FIREFOX_LINUX.into(), FIREFOX_LINUX),
    };

    let mut inif = Ini::new();
    inif.load(bs.to_str().unwrap())
        .expect("get ff profiles failed");

    let mut section = String::new();

    for (sect, val) in inif.get_map().unwrap().into_iter() {
        if sect.starts_with("install") {
            for (sect, val) in val {
                if sect == "default" {
                    section = val.unwrap_or_default();
                }
            }
        }
    }
    debug!("section: {}", section);

    let mut cookies_db = home_dir().expect("get home dir failed");
    cookies_db.push(format!("{}/{}/cookies.sqlite", browser, section));

    Ok(cookies_db)
}

async fn get_ff_conn(borwser: &str) -> Result<DatabaseConnection> {
    let cookie_dir = get_cookie_file(borwser).await?;

    let db_conn_str = format!(
        "sqlite:{}?mode=rwc",
        cookie_dir
            .to_string_lossy()
            .to_string()
    );

    debug!("database dir: {}", &db_conn_str);

    let db = Database::connect(db_conn_str)
        .await
        .into_diagnostic()?;

    Ok(db)
}

async fn query_cookie(borwser: &str) -> Result<Vec<Model>> {
    let db = get_ff_conn(borwser).await?;
    let host = global_user_config()
        .urls
        .origin
        .split("//")
        .collect::<Vec<&str>>()[1];
    let res = MozCookies::find()
        .filter(moz_cookies::Column::Host.contains(host))
        .all(&db)
        .await
        .into_diagnostic()?;

    Ok(res)
}

pub async fn get_ff_session_csrf(borwser: &str) -> Result<Cookies> {
    let cookies = query_cookie(borwser).await?;

    let mut res = Cookies::default();
    for cookie in cookies {
        if cookie
            .name
            .as_ref()
            .unwrap_or(&"".to_string())
            == "csrftoken"
        {
            res.csrf = cookie.value.unwrap_or_default();
        } else if cookie
            .name
            .as_ref()
            .unwrap_or(&"".to_string())
            == "LEETCODE_SESSION"
        {
            res.session = cookie.value.unwrap_or_default();
        }
    }
    Ok(res)
}
