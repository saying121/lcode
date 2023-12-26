mod dao;
mod entities;

use std::path::PathBuf;

use miette::{IntoDiagnostic, Result};
use secret_service::{EncryptionType, SecretService};
use tracing::debug;

use crate::{Browser, Cookies};

const CHROME_STORAGE_NAME: &str = "Chrome Safe Storage";
const EDGE_STORAGE_NAME: &str = "Chrome Safe Storage";
// const CHROMIUM_STORAGE_NAME: &str = "Chromium Safe Storage";

pub const CHROME_LINUX: &str = "google-chrome/Default/Cookies";
pub const CHROME_LINUX1: &str = "google-chrome/Profile 1/Cookies";
// pub const CHROME_LINUX: &str = "google-chrome/Guest Profile/Cookies";
// pub const CHROME_LINUX: &str = "google-chrome/System Profile/Cookies";

pub const CHROME_MAC: &str = "Google/Chrome/Default/Cookies";
pub const CHROME_WIN: &str = "Local\\Google\\Chrome\\User Data\\Default\\Cookies";

pub const EDGE_LINUX: &str = "microsoft-edge/Default/Cookies";
pub const EDGE_MAC: &str = "Microsoft Edge/Default/Cookies";
pub const EDGE_WIN: &str = "Microsoft\\Edge\\User Data\\Default\\Cookies";

pub fn get_browser_cookies_path(browser: Browser) -> PathBuf {
    #[cfg(target_os = "linux")]
    let v = match browser {
        Browser::Edge => EDGE_LINUX,
        Browser::Chrome => CHROME_LINUX1,
        _ => EDGE_LINUX,
    };
    #[cfg(target_os = "macos")]
    let v = match browser {
        Browser::Edge => EDGE_MAC,
        Browser::Chrome => CHROME_MAC,
        _ => EDGE_MAC,
    };
    #[cfg(target_os = "windows")]
    let v = match browser {
        Browser::Edge => EDGE_WIN,
        Browser::Chrome => CHROME_WIN,
        _ => EDGE_WIN,
    };
    let mut cookie_dir = dirs::config_dir().expect("get config dir failed");
    cookie_dir.push(v);

    if browser == Browser::Chrome && !cookie_dir.exists() {
        cookie_dir = dirs::config_dir().expect("get config dir failed");
        cookie_dir.push(CHROME_LINUX);
    }
    cookie_dir
}

/// get `LEETCODE_SESSION` and `csrftoken`
///
/// * `browser`: `"chrome"`, `"edge"`
// , brave, opera, vivaldi, chromium
pub async fn get_session_csrf(browser: Browser, host: &str) -> Result<Cookies> {
    let cookies = dao::query_cookie(browser, host).await?;

    let mut res = Cookies::default();
    for cookie in cookies {
        if cookie.name == "csrftoken" {
            res.csrf = decrypt_cookies(&cookie.encrypted_value, browser).await?;
        }
        else if cookie.name == "LEETCODE_SESSION" {
            res.session = decrypt_cookies(&cookie.encrypted_value, browser).await?;
        }
    }

    Ok(res)
}

/// from `secret_service` get pass
async fn get_pass(browser: Browser) -> Result<Vec<u8>> {
    let default_pass = Ok(b"peanuts".to_vec());
    // initialize secret service (dbus connection and encryption session)
    let Ok(ss) = SecretService::connect(EncryptionType::Dh).await
    else {
        return default_pass;
    };
    // get default collection
    let Ok(collection) = ss.get_default_collection().await
    else {
        return default_pass;
    };
    let coll = collection
        .get_all_items()
        .await
        .into_diagnostic()?;
    let label = match browser {
        Browser::Edge => EDGE_STORAGE_NAME,
        Browser::Chrome => CHROME_STORAGE_NAME,
        _ => "",
    };
    let mut res = vec![];
    for i in coll {
        if i.get_label()
            .await
            .into_diagnostic()?
            == label
        {
            res = i
                .get_secret()
                .await
                .into_diagnostic()?;
        }
    }
    debug!("res: {}", String::from_utf8_lossy(&res).to_string());
    if res.is_empty() {
        res = b"peanuts".to_vec();
    }
    debug!("done res: {}", String::from_utf8_lossy(&res).to_string());

    Ok(res)
}

pub async fn decrypt_cookies(be_decrypte: &[u8], browser: Browser) -> Result<String> {
    use openssl::{hash::MessageDigest, pkcs5::pbkdf2_hmac, symm};

    let mut key = [32_u8; 16];
    let pass = get_pass(browser).await?;

    pbkdf2_hmac(&pass, b"saltysalt", 1, MessageDigest::sha1(), &mut key).into_diagnostic()?;

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
        .update(
            be_decrypte
                .get(3..)
                .expect("crypto error"),
            &mut res,
        )
        .into_diagnostic()?;
    decrypter
        .finalize(&mut res)
        .into_diagnostic()?;

    res.retain(|v| v >= &20);

    Ok(String::from_utf8_lossy(&res).to_string())
}
