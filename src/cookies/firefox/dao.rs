use miette::{IntoDiagnostic, Result};
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::debug;

use super::entities::{
    moz_cookies::{self, Model},
    prelude::*,
};
use crate::config::global::glob_user_config;

async fn get_ff_conn(borwser: &str) -> Result<DatabaseConnection> {
    let cookie_dir = super::get_cookie_path(borwser).await?;

    let db_conn_str = format!("sqlite:{}?mode=rwc", cookie_dir.to_string_lossy());

    debug!("database dir: {}", &db_conn_str);

    let db = Database::connect(db_conn_str)
        .await
        .into_diagnostic()?;

    Ok(db)
}

pub async fn query_cookie(borwser: &str) -> Result<Vec<Model>> {
    let db = get_ff_conn(borwser).await?;
    let host = glob_user_config()
        .urls
        .origin
        .split("//")
        .collect::<Vec<&str>>()[1];
    let res = MozCookies::find()
        .filter(moz_cookies::Column::Host.contains(host))
        .all(&db)
        .await
        .into_diagnostic()?;

    Ok(res)
}
