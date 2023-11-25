pub mod query_topic_tags;
pub mod save_info;

use std::sync::OnceLock;

use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, Database, DatabaseConnection,
    EntityTrait, ModelTrait, QueryFilter, Schema,
};
use tokio::{fs::create_dir_all, join};
use tracing::{debug, error};

use crate::entities::prelude::*;
use crate::{config::global, entities::*, leetcode::IdSlug};

#[async_trait]
pub trait InsertToDB: std::marker::Sized {
    type Value: Into<sea_orm::Value> + Send;
    type Entity: EntityTrait;
    type Model: ModelTrait;
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity> + std::marker::Send;

    fn to_active_model(&self, value: Self::Value) -> Self::ActiveModel;
    async fn insert_to_db(&self, value: Self::Value);
}
#[async_trait]
pub trait InsertIntoDB: std::marker::Sized {
    type Value: Into<sea_orm::Value> + Send;
    type Entity: EntityTrait;
    type Model: ModelTrait;
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity> + std::marker::Send;

    fn into_active_model(self, value: Self::Value) -> Self::ActiveModel;
    async fn insert_into_db(self, value: Self::Value);
}

pub static DB: OnceLock<DatabaseConnection> = OnceLock::new();
/// # Initialize the db connection
pub fn glob_db() -> &'static DatabaseConnection {
    DB.get_or_init(|| {
        pollster::block_on(async {
            let db = conn_db().await.unwrap_or_default();

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

            let stmt_newidx = builder.build(
                schema
                    .create_table_from_entity(NewIndexDB)
                    .if_not_exists(),
            );
            let stmt_topic = builder.build(
                schema
                    .create_table_from_entity(TopicTagsDB)
                    .if_not_exists(),
            );
            let stmt_qs_tag = builder.build(
                schema
                    .create_table_from_entity(QsTagDB)
                    .if_not_exists(),
            );
            // new table
            let res = join!(
                db.execute(stmt_index),
                db.execute(stmt_detail),
                db.execute(stmt_newidx),
                db.execute(stmt_topic),
                db.execute(stmt_qs_tag)
            );

            if let Err(err) = res.0 {
                error!("{}", err);
            }
            if let Err(err) = res.1 {
                error!("{}", err);
            }
            if let Err(err) = res.2 {
                error!("{}", err);
            }
            if let Err(err) = res.3 {
                error!("{}", err);
            }
            if let Err(err) = res.4 {
                error!("{}", err);
            }

            db
        })
    })
}
// get database connection
pub async fn conn_db() -> Result<DatabaseConnection> {
    let db_dir = global::glob_database_path();
    create_dir_all(
        db_dir
            .parent()
            .unwrap_or_else(|| global::glob_code_dir()),
    )
    .await
    .into_diagnostic()?;

    let db_conn_str = format!("sqlite:{}?mode=rwc", db_dir.to_string_lossy());
    debug!("database dir: {}", &db_conn_str);

    let db = Database::connect(db_conn_str)
        .await
        .into_diagnostic()?;

    Ok(db)
}

/// Find the problem, if there are more than one, return one
///
/// * `idslug`: id or title
pub async fn get_question_index_exact(idslug: &IdSlug) -> Result<index::Model> {
    match idslug {
        IdSlug::Id(id) => {
            let models = Index::find_by_id(*id)
                .one(glob_db())
                .await
                .into_diagnostic()?
                .unwrap_or_default();
            debug!("get value {:#?}", models);
            Ok(models)
        }
        IdSlug::Slug(slug) => {
            let models = Index::find()
                .filter(index::Column::QuestionTitleSlug.eq(slug))
                .one(glob_db())
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
pub async fn get_question_index(idslug: IdSlug) -> Result<Vec<index::Model>> {
    match idslug {
        IdSlug::Id(id) => {
            let models = Index::find_by_id(id)
                .all(glob_db())
                .await
                .into_diagnostic()?;
            debug!("get value {:#?}", models);
            Ok(models)
        }
        IdSlug::Slug(slug) => {
            let models = Index::find()
                .filter(index::Column::QuestionTitleSlug.contains(slug))
                .all(glob_db())
                .await
                .into_diagnostic()?;
            debug!("res {:#?}", models);

            Ok(models)
        }
    }
}

pub async fn query_all_index() -> Result<Vec<index::Model>> {
    let models = Index::find()
        .all(glob_db())
        .await
        .into_diagnostic()?;

    Ok(models)
}
