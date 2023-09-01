mod entities;
mod dao;

use std::path::PathBuf;

use miette::{IntoDiagnostic, Result};
use openssl::{hash::MessageDigest, pkcs5::pbkdf2_hmac, symm};
use tracing::debug;

const CHROME_STORAGE_NAME: &str = "Chrome Safe Storage";
const EDGE_STORAGE_NAME: &str = "Chrome Safe Storage";
// const CHROMIUM_STORAGE_NAME: &str = "Chromium Safe Storage";

// pub const CHROME_LINUX: &str = "google-chrome/Default/Cookies";
pub const CHROME_LINUX: &str = "google-chrome/Profile 1/Cookies";
// pub const CHROME_LINUX: &str = "google-chrome/Guest Profile/Cookies";
// pub const CHROME_LINUX: &str = "google-chrome/System Profile/Cookies";

pub const CHROME_MAC: &str = "Google/Chrome/Default/Cookies";
pub const CHROME_WIN: &str = "Local\\Google\\Chrome\\User Data\\Default\\Cookies";

pub const EDGE_LINUX: &str = "microsoft-edge/Default/Cookies";
pub const EDGE_MAC: &str = "Microsoft Edge/Default/Cookies";
pub const EDGE_WIN: &str = "Microsoft\\Edge\\User Data\\Default\\Cookies";

pub fn get_browser_cookies_path(browser: &str) -> PathBuf {
    #[cfg(target_os = "linux")]
    let v = match browser {
        "edge" => EDGE_LINUX,
        "chrome" => CHROME_LINUX,
        _ => EDGE_LINUX,
    };
    #[cfg(target_os = "macos")]
    let v = match browser {
        "edge" => EDGE_MAC,
        "chrome" => CHROME_MAC,
        _ => EDGE_MAC,
    };
    #[cfg(target_os = "windows")]
    let v = match browser {
        "edge" => EDGE_WIN,
        "chrome" => CHROME_WIN,
        _ => EDGE_WIN,
    };
    let mut cookie_dir = dirs::config_dir().expect("get config dir failed");
    cookie_dir.push(v);
    cookie_dir
}

/// get `LEETCODE_SESSION` and `csrftoken`
///
/// * `browser`: chrome, edge
// , brave, opera, vivaldi, chromium
pub async fn get_session_csrf(
    browser: &str,
) -> Result<crate::config::user_nest::Cookies> {
    let cookies = dao::query_cookie(browser).await?;

    let mut res = crate::config::user_nest::Cookies::default();
    for cookie in cookies {
        if cookie.name == "csrftoken" {
            res.csrf = decrypt_cookies(&cookie.encrypted_value, browser).await?;
        } else if cookie.name == "LEETCODE_SESSION" {
            res.session = decrypt_cookies(&cookie.encrypted_value, browser).await?;
        }
    }

    Ok(res)
}

/// from secret_service get pass
async fn get_pass(browser: &str) -> Result<Vec<u8>> {
    // dbus_session.
    use secret_service::EncryptionType;
    use secret_service::SecretService;
    // initialize secret service (dbus connection and encryption session)
    let ss = SecretService::connect(EncryptionType::Dh)
        .await
        .unwrap();
    // get default collection
    let collection = ss
        .get_default_collection()
        .await
        .unwrap();
    let coll = collection
        .get_all_items()
        .await
        .into_diagnostic()?;
    let label = match browser {
        "edge" => EDGE_STORAGE_NAME,
        "chrome" => CHROME_STORAGE_NAME,
        _ => "",
    };
    let mut res = vec![];
    for i in coll {
        if &i
            .get_label()
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
    if res.len() == 0 {
        res = b"peanuts".to_vec();
    }
    debug!("done res: {}", String::from_utf8_lossy(&res).to_string());

    Ok(res)
}

pub async fn decrypt_cookies(be_decrypte: &Vec<u8>, browser: &str) -> Result<String> {
    let mut key = [32_u8; 16];
    let pass = get_pass(browser).await?;

    pbkdf2_hmac(&pass, b"saltysalt", 1, MessageDigest::sha1(), &mut key)
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
