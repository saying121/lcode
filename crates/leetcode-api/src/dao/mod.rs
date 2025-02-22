pub mod query;
pub mod save_info;

use std::future::Future;

use lcode_config::global::G_DATABASE_PATH;
use miette::{IntoDiagnostic, Result};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, Database, DatabaseConnection, EntityTrait, IntoActiveModel,
    ModelTrait, Schema, sea_query::OnConflict,
};
use tokio::{join, sync::OnceCell};
use tracing::{debug, error};

use crate::entities::{prelude::*, *};

pub trait InsertToDB: std::marker::Sized {
    type Value: Into<sea_orm::Value> + Send;
    type Entity: EntityTrait;
    type Model: ModelTrait + Default + IntoActiveModel<Self::ActiveModel>;
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity>
        + std::marker::Send
        + std::convert::From<Self::Model>;

    fn to_model(&self, _info: Self::Value) -> Self::Model {
        Self::Model::default()
    }
    /// Insert with extra logic
    ///
    /// * `_info`: extra info
    fn insert_to_db(&mut self, _info: Self::Value) -> impl Future<Output = ()> + Send {
        async {}
    }
    fn to_activemodel(&self, value: Self::Value) -> Self::ActiveModel {
        self.to_model(value).into_active_model()
    }
    /// Insert One
    ///
    /// * `_info`: extra info
    fn insert_one(&self, info: Self::Value) -> impl Future<Output = ()> + Send {
        let pat = self.to_activemodel(info);
        async {
            if let Err(err) = Self::Entity::insert(pat)
                .on_conflict(Self::on_conflict())
                .exec(glob_db().await)
                .await
            {
                error!("{}", err);
            }
        }
    }
    fn insert_many(value: Vec<Self::ActiveModel>) -> impl Future<Output = ()> + Send {
        async {
            if let Err(err) = Self::Entity::insert_many(value)
                .on_conflict(Self::on_conflict())
                .exec(glob_db().await)
                .await
            {
                error!("{}", err);
            }
        }
    }
    fn on_conflict() -> OnConflict;
}

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();
/// # Initialize the db connection
pub async fn glob_db() -> &'static DatabaseConnection {
    DB.get_or_init(|| async {
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

        let stmt_newindexdb = builder.build(
            schema
                .create_table_from_entity(NewIndexDB)
                .if_not_exists(),
        );
        let stmt_topictagsdb = builder.build(
            schema
                .create_table_from_entity(TopicTagsDB)
                .if_not_exists(),
        );
        let stmt_qstagdb = builder.build(
            schema
                .create_table_from_entity(QsTagDB)
                .if_not_exists(),
        );

        // new table
        let res = join!(
            db.execute(stmt_index),
            db.execute(stmt_detail),
            db.execute(stmt_newindexdb),
            db.execute(stmt_topictagsdb),
            db.execute(stmt_qstagdb)
        );

        macro_rules! log_errors {
            ($($res:expr),*) => {
                $(
                    if let Err(err) = $res {
                        error!("{}", err);
                    }
                )*
            };
        }
        log_errors!(res.0, res.1, res.2, res.3, res.4);

        db
    })
    .await
}
/// get database connection
async fn conn_db() -> Result<DatabaseConnection> {
    let db_conn_str = format!("sqlite:{}?mode=rwc", G_DATABASE_PATH.to_string_lossy());
    debug!("database dir: {}", &db_conn_str);

    Database::connect(db_conn_str)
        .await
        .into_diagnostic()
}
