use miette::{IntoDiagnostic, Result};
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use tokio::task;
use tracing::debug;

use super::entities::{
    moz_cookies::{self, Model},
    prelude::*,
};
use crate::Browser;

async fn get_ff_conn(borwser: Browser) -> Result<DatabaseConnection> {
    let cookie_dir = task::spawn_blocking(move || super::get_cookie_path(borwser))
        .await
        .into_diagnostic()?;

    let db_conn_str = format!("sqlite:{}?mode=rwc", cookie_dir.to_string_lossy());

    debug!("database dir: {}", &db_conn_str);

    let db = Database::connect(db_conn_str)
        .await
        .into_diagnostic()?;

    Ok(db)
}

pub async fn query_cookie(borwser: Browser, host: &str) -> Result<Vec<Model>> {
    let db = get_ff_conn(borwser).await?;
    let res = MozCookies::find()
        .filter(moz_cookies::Column::Host.contains(host))
        .all(&db)
        .await
        .into_diagnostic()?;

    Ok(res)
}
