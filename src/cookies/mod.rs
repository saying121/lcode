mod entities;

use std::collections::HashMap;

use miette::{IntoDiagnostic, Result};
use openssl::{hash, pkcs5::pbkdf2_hmac, symm};
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::debug;

use crate::config::global::global_user_config;

use self::entities::cookies::{self, Model};
use self::entities::prelude::*;

const CHROME_LINUX: &str = "google-chrome/Default/Cookies";
const EDGE_LINUX: &str = "microsoft-edge/Default/Cookies";

pub async fn get_cookie_conn() -> Result<DatabaseConnection> {
    let mut cookie_dir = dirs::config_dir().expect("get config dir failed");
    cookie_dir.push(CHROME_LINUX);
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

    Ok(db)
}

pub async fn query_cookie() -> Result<Vec<Model>> {
    let db = get_cookie_conn().await?;
    let host = global_user_config()
        .url_suffix
        .origin
        .split("//")
        .collect::<Vec<&str>>()[1];
    let res = Cookies::find()
        .filter(cookies::Column::HostKey.contains(host))
        .all(&db)
        .await
        .into_diagnostic()?;

    Ok(res)
}

pub async fn get_session_csrf() -> Result<HashMap<String, Vec<u8>>> {
    let cookies = query_cookie().await?;
    // csrftoken
    // LEETCODE_SESSION
    let mut res = HashMap::new();
    for cookie in cookies {
        if cookie.name == "csrftoken" {
            res.insert("csrftoken".into(), cookie.encrypted_value);
        } else if cookie.name == "LEETCODE_SESSION" {
            res.insert("LEETCODE_SESSION".into(), cookie.encrypted_value);
        }
    }
    Ok(res)
}

pub async fn decrypt_cookies(be_decrypte: &Vec<u8>) -> Result<String> {
    let mut key = [0_u8; 16];
    pbkdf2_hmac(
        b"peanuts",
        b"saltysalt",
        1,
        hash::MessageDigest::sha1(),
        &mut key,
    )
    .into_diagnostic()?;

    let iv = vec![0_u8; 16];
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
    decrypter
        .update(&be_decrypte[3..], &mut res)
        .into_diagnostic()?;
    decrypter
        .finalize(&mut res)
        .into_diagnostic()?;

    // Ok(())
    Ok(String::from_utf8_lossy(&res.to_vec()).to_string())
}
