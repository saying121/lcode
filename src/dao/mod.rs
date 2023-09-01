pub mod save_info;

use miette::{Error, IntoDiagnostic, Result};
use sea_orm::{
    ColumnTrait, ConnectionTrait, Database, DatabaseConnection, EntityTrait, QueryFilter,
    Schema,
};
use tokio::{fs::create_dir_all, join};
use tracing::{debug, trace};

use crate::entities::prelude::*;
use crate::{config::global, entities::*, leetcode::IdSlug};

// get database connection
pub async fn conn_db() -> Result<DatabaseConnection, Error> {
    let db_dir = global::glob_database_dir();
    create_dir_all(
        db_dir
            .parent()
            .unwrap_or_else(|| global::glob_code_dir()),
    )
    .await
    .into_diagnostic()?;

    let db_conn_str = format!(
        "sqlite:{}?mode=rwc",
        db_dir
            .to_string_lossy()
            .to_string()
    );
    debug!("database dir: {}", &db_conn_str);

    let db = Database::connect(db_conn_str)
        .await
        .into_diagnostic()?;
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let stmt_index = builder.build(
        schema
            .create_table_from_entity(Index)
            .if_not_exists(),
    );
    let stmt_detail = builder.build(
        schema
            .create_table_from_entity(Detail)
            .if_not_exists(),
    );

    // new table
    let (index_res, detail_res) = join!(db.execute(stmt_index), db.execute(stmt_detail));
    let (index_exec, detail_exec) =
        (index_res.into_diagnostic()?, detail_res.into_diagnostic()?);

    trace!("create database: {:?},{:?}", index_exec, detail_exec);

    Ok(db)
}

/// Find the problem, if there are more than one, return one
///
/// * `idslug`: id or title
pub async fn get_question_index_exact(idslug: &IdSlug) -> Result<index::Model, Error> {
    let db = conn_db().await?;
    match idslug {
        IdSlug::Id(id) => {
            let models = Index::find_by_id(*id)
                .one(&db)
                .await
                .into_diagnostic()?
                .unwrap_or_default();
            debug!("get value {:#?}", models);
            Ok(models)
        }
        IdSlug::Slug(slug) => {
            let models = Index::find()
                .filter(index::Column::QuestionTitleSlug.eq(slug))
                .one(&db)
                .await
                .into_diagnostic()?
                .unwrap_or_default();
            debug!("res {:#?}", models);

            Ok(models)
        }
    }
}

/// Find the problem, if there are more than one, return multiple
///
/// * `idslug`: id or title
pub async fn get_question_index(idslug: IdSlug) -> Result<Vec<index::Model>, Error> {
    let db = conn_db().await?;
    match idslug {
        IdSlug::Id(id) => {
            let models = Index::find_by_id(id)
                .all(&db)
                .await
                .into_diagnostic()?;
            debug!("get value {:#?}", models);
            Ok(models)
        }
        IdSlug::Slug(slug) => {
            let models = Index::find()
                .filter(index::Column::QuestionTitleSlug.contains(slug))
                .all(&db)
                .await
                .into_diagnostic()?;
            debug!("res {:#?}", models);

            Ok(models)
        }
    }
}

pub async fn query_all_index() -> Result<Vec<index::Model>, Error> {
    let db = conn_db().await?;

    let models = Index::find()
        .all(&db)
        .await
        .into_diagnostic()?;

    Ok(models)
}
