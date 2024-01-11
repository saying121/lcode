mod dao;
mod entities;
mod path;

use miette::{IntoDiagnostic, Result};
use secret_service::{EncryptionType, SecretService};
use tracing::debug;

use self::entities::cookies;
use crate::{Browser, Cookies};

/// get `LEETCODE_SESSION` and `csrftoken`
///
/// * `browser`: `"chrome"`, `"edge"`
// , brave, opera, vivaldi, chromium
pub async fn get_session_csrf(browser: Browser, host: &str) -> Result<Cookies> {
    let mut cookies = dao::query_cookie(browser, host).await?;

    let mut res = Cookies::default();
    // the `encrypted_value` start with `v10`, so use `[3..]`
    for cookie in &mut cookies {
        if cookie.name == "csrftoken" {
            decrypt_cookies(&mut cookie.encrypted_value, browser).await?;
            res.csrf = String::from_utf8_lossy(&cookie.encrypted_value[3..]).to_string();
        }
        else if cookie.name == "LEETCODE_SESSION" {
            decrypt_cookies(&mut cookie.encrypted_value, browser).await?;
            res.session = String::from_utf8_lossy(&cookie.encrypted_value[3..]).to_string();
        }
    }

    Ok(res)
}
pub async fn get_all_cookies(browser: Browser, host: &str) -> Result<Vec<cookies::Model>> {
    let mut cookies = dao::query_cookie(browser, host).await?;

    for cookie in &mut cookies {
        decrypt_cookies(&mut cookie.encrypted_value, browser).await?;
    }

    Ok(cookies)
}

const CHROME_STORAGE_NAME: &str = "Chrome Safe Storage";
const EDGE_STORAGE_NAME: &str = "Chrome Safe Storage";
// const CHROMIUM_STORAGE_NAME: &str = "Chromium Safe Storage";

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
        _ => CHROME_STORAGE_NAME,
    };
    let mut res = vec![];
    for i in coll {
        if let Ok(l) = i.get_label().await {
            if l == label {
                res = i
                    .get_secret()
                    .await
                    .into_diagnostic()?;
            }
        }
    }
    debug!("res: {}", String::from_utf8_lossy(&res).to_string());
    if res.is_empty() {
        return default_pass;
    }

    Ok(res)
}

pub async fn decrypt_cookies(be_decrypte: &mut Vec<u8>, browser: Browser) -> Result<()> {
    use aes::cipher::{block_padding, BlockDecryptMut, KeyIvInit};
    use pbkdf2::pbkdf2_hmac;

    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

    let mut key = [0_u8; 16];
    let pass = get_pass(browser).await?;

    pbkdf2_hmac::<sha1::Sha1>(&pass, b"saltysalt", 1, &mut key);

    let iv = [32_u8; 16];

    let decrypter = Aes128CbcDec::new(&key.into(), &iv.into());

    decrypter
        .decrypt_padded_mut::<block_padding::NoPadding>(&mut be_decrypte[3..])
        .unwrap();

    be_decrypte.retain(|v| v >= &32);

    Ok(())
}
