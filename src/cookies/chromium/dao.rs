use miette::{IntoDiagnostic, Result};
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::debug;

use super::{
    entities::cookies::{self, Model},
    get_browser_cookies_path,
};
use crate::{
    config::global::glob_user_config, cookies::chromium::entities::prelude::Cookies,
};

/// get database connect
///
/// * `browser`: `edge`, `chrome`
pub(crate) async fn get_conn(browser: &str) -> Result<DatabaseConnection> {
    let cookie_path = get_browser_cookies_path(browser);

    let db_conn_str = format!(
        "sqlite:{}?mode=rwc",
        cookie_path
            .to_string_lossy()
            .to_string()
    );

    let db = Database::connect(db_conn_str)
        .await
        .into_diagnostic()?;

    Ok(db)
}

pub(crate) async fn query_cookie(browser: &str) -> Result<Vec<Model>> {
    let db = get_conn(browser).await?;

    let host = glob_user_config()
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

    Ok(res)
}
