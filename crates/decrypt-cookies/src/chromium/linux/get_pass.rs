use miette::{IntoDiagnostic, Result};
use secret_service::{EncryptionType, SecretService};
use tracing::debug;

use super::{CHROME_STORAGE_NAME, EDGE_STORAGE_NAME};
use crate::Browser;

/// from `secret_service` get pass
pub async fn get_pass(browser: Browser) -> Result<Vec<u8>> {
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
        let Ok(l) = i.get_label().await
        else {
            continue;
        };
        if l == label {
            res = i.get_secret().await.into_diagnostic()?;
        }
    }
    debug!("res: {}", String::from_utf8_lossy(&res).to_string());
    if res.is_empty() {
        return default_pass;
    }

    Ok(res)
}
