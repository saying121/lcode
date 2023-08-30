use std::collections::HashMap;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use miette::{IntoDiagnostic, Result};
use openssl::{hash::MessageDigest, pkcs5::pbkdf2_hmac, symm};
use sea_orm::{Database, DatabaseConnection};
use tracing::debug;

use crate::config::global::global_user_config;
use crate::cookies::chromium_base_entities::{
    cookies::{self, Model},
    prelude::*,
};

// now just support edge
// const CHROME_LINUX: &str = "google-chrome/Default/Cookies";
pub const EDGE_LINUX: &str = "microsoft-edge/Default/Cookies";
pub const EDGE_MAC: &str = "Microsoft Edge{channel}/Default/Cookies";
pub const EDGE_WIN: &str = "Microsoft\\Edge\\User Data\\Default\\Cookies";

async fn get_edge_conn() -> Result<DatabaseConnection> {
    let browser = match std::env::consts::OS {
        "linux" => EDGE_LINUX,
        "macos" => EDGE_MAC,
        "windows" => EDGE_WIN,
        _ => EDGE_LINUX,
    };
    debug!("cookie: {}", browser);

    let mut cookie_dir = dirs::config_dir().expect("get config dir failed");
    cookie_dir.push(browser);
    debug!("{:?}", cookie_dir);
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
    debug!("done");

    Ok(db)
}

async fn query_cookie() -> Result<Vec<Model>> {
    let db = get_edge_conn().await?;
    let host = global_user_config()
        .urls
        .origin
        .split("//")
        .collect::<Vec<&str>>()[1];
    debug!("host: {}", host);
    let res = Cookies::find()
        .filter(cookies::Column::HostKey.contains(host))
        .all(&db)
        .await
        .into_diagnostic()?;
    debug!("query res: {:?}", res);

    Ok(res)
}

pub async fn get_chrom_session_csrf() -> Result<crate::config::user_nest::Cookies> {
    debug!("start");
    let cookies = query_cookie().await?;
    debug!("{:?}", cookies);

    let mut csrf_session = HashMap::new();
    for cookie in cookies {
        if cookie.name == "csrftoken" {
            csrf_session.insert("csrftoken".to_string(), cookie.encrypted_value);
        } else if cookie.name == "LEETCODE_SESSION" {
            csrf_session.insert("LEETCODE_SESSION".to_string(), cookie.encrypted_value);
        }
    }
    let csrf = decrypt_cookies(
        csrf_session
            .get("csrftoken")
            .unwrap(),
    )
    .await?;

    let session = decrypt_cookies(
        csrf_session
            .get("LEETCODE_SESSION")
            .unwrap(),
    )
    .await?;

    let mut res = crate::config::user_nest::Cookies::default();
    res.csrf = csrf;
    res.session = session;

    Ok(res)
}

pub async fn decrypt_cookies(be_decrypte: &Vec<u8>) -> Result<String> {
    let mut key = [32_u8; 16];

    pbkdf2_hmac(b"peanuts", b"saltysalt", 1, MessageDigest::sha1(), &mut key)
        .into_diagnostic()?;

    let iv = vec![32_u8; 16];

    let mut decrypter = symm::Crypter::new(
        symm::Cipher::aes_128_cbc(),
        symm::Mode::Decrypt,
        &key,
        Some(&iv),
    )
    .into_diagnostic()?;

    let data_len = be_decrypte.len() - 3;
    let block_size = symm::Cipher::aes_128_cbc().block_size();
    let mut res = vec![0; data_len + block_size];

    decrypter.pad(false);
    let _num = decrypter
        .update(&be_decrypte[3..], &mut res)
        .into_diagnostic()?;
    decrypter
        .finalize(&mut res)
        .into_diagnostic()?;

    res.retain(|v| v >= &20);

    Ok(String::from_utf8_lossy(&res).to_string())
}
