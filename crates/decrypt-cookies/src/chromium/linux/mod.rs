mod path;
mod get_pass;
mod crypto;

use miette::{IntoDiagnostic, Result};
use tokio::task;

use super::{dao, entities::cookies};
use crate::{Browser, Cookies};

const CHROME_STORAGE_NAME: &str = "Chrome Safe Storage";
const EDGE_STORAGE_NAME: &str = "Microsoft Edge Safe Storage";
const CHROMIUM_STORAGE_NAME: &str = "Chromium Safe Storage";

/// get `LEETCODE_SESSION` and `csrftoken`
///
/// * `browser`: `"chrome"`, `"edge"`
// , brave, opera, vivaldi, chromium
pub async fn get_session_csrf(browser: Browser, host: &str) -> Result<Cookies> {
    let mut cookies = dao::query_cookie(browser, host).await?;
    let pass = get_pass::get_pass(browser).await?;

    task::spawn_blocking(move || {
        let mut res = Cookies::default();
        // the `encrypted_value` start with `v10`, so use `[3..]`
        for cookie in &mut cookies {
            if cookie.name == "csrftoken" {
                crypto::decrypt_cookies(&mut cookie.encrypted_value, &pass)?;
                tracing::trace!("{:?}", &cookie.encrypted_value[3..]);
                res.csrf = String::from_utf8_lossy(&cookie.encrypted_value[3..]).to_string();
            }
            else if cookie.name == "LEETCODE_SESSION" {
                crypto::decrypt_cookies(&mut cookie.encrypted_value, &pass)?;
                res.session = String::from_utf8_lossy(&cookie.encrypted_value[3..]).to_string();
            }
        }
        Ok(res)
    })
    .await
    .into_diagnostic()?
}
pub async fn get_all_cookies(browser: Browser, host: &str) -> Result<Vec<cookies::Model>> {
    let mut cookies = dao::query_cookie(browser, host).await?;
    let pass = get_pass::get_pass(browser).await?;

    task::spawn_blocking(move || {
        for cookie in &mut cookies {
            crypto::decrypt_cookies(&mut cookie.encrypted_value, &pass)?;
        }
        Ok(cookies)
    })
    .await
    .into_diagnostic()?
}
