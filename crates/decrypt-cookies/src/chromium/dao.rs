use miette::{IntoDiagnostic, Result};
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};

use super::{
    entities::{
        cookies::{self, Model},
        prelude::*,
    },
    path::get_browser_cookies_path,
};
use crate::Browser;

/// get database connect
///
/// * `browser`: `edge`, `chrome`
pub async fn get_conn(browser: Browser) -> Result<DatabaseConnection> {
    let cookie_path = get_browser_cookies_path(browser);

    let db_conn_str = format!("sqlite:{}?mode=rwc", cookie_path.to_string_lossy());

    let db = Database::connect(db_conn_str)
        .await
        .into_diagnostic()?;

    Ok(db)
}

pub async fn query_cookie(browser: Browser, host: &str) -> Result<Vec<Model>> {
    let db = get_conn(browser).await?;

    let res = CookiesDB::find()
        .filter(cookies::Column::HostKey.contains(host))
        .all(&db)
        .await
        .into_diagnostic()?;

    Ok(res)
}
