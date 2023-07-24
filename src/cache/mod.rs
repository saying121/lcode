mod problem_struct;

use crate::{
    config::{self, Config, User, CATEGORIES},
    entities::{prelude::*, *},
};
use miette::{Error, IntoDiagnostic, Result};
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use sea_orm::{ConnectionTrait, Database, EntityTrait, Schema};
use serde_json::Value;
use std::time::Duration;
use tokio::fs::create_dir_all;

pub struct Cache {
    client: Client,
    headers: HeaderMap,
}

impl Cache {
    /// new Cache
    pub async fn new() -> Result<Self, Error> {
        let client = ClientBuilder::new()
            .gzip(true)
            .connect_timeout(Duration::from_secs(30))
            .build()
            .into_diagnostic()?;
        let config = Config::new().await?;

        Ok(Cache {
            client,
            headers: config.headers,
        })
    }

    /// get leetcode index
    ///
    /// # Panics
    ///
    /// - json parser error
    ///
    /// # Errors
    ///
    /// - network error
    /// - leetcode url change
    /// * `force`: when true will force update
    pub async fn get_all_problems(&self) -> Result<(), Error> {
        let db_dir = config::init_database_dir();
        create_dir_all(db_dir.parent().unwrap())
            .await
            .into_diagnostic()?;
        let db_str = format!(
            "sqlite:{}?mode=rwc",
            db_dir
                .to_string_lossy()
                .to_string()
        );
        #[cfg(debug_assertions)]
        dbg!(&db_str);

        let db = Database::connect(db_str)
            .await
            .into_diagnostic()?;
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        let stmt = builder.build(
            schema
                .create_table_from_entity(Problem)
                .if_not_exists(),
        );
        // new database
        db.execute(stmt)
            .await
            .into_diagnostic()?;

        let df_v = Value::default();
        for category in CATEGORIES {
            let new_url = User::default()
                .all_problem_api
                .replace("$category", category);
            let new_headers = Config::mod_headers(
                self.headers.clone(),
                vec![("Referer", &new_url)],
            )?;

            let problems_resp_json: Value = self
                .client
                .get(new_url)
                .headers(new_headers)
                .send()
                .await
                .into_diagnostic()?
                .json()
                .await
                .into_diagnostic()?;

            // Get the part of the question
            let problems_json = problems_resp_json
                .get("stat_status_pairs")
                .unwrap_or(&df_v)
                .as_array()
                .unwrap();

            for problem in problems_json {
                let pb: problem_struct::Problem =
                    serde_json::from_value(problem.clone())
                        .into_diagnostic()?;

                #[rustfmt::skip]
                let pb_db = problem::ActiveModel {
                    question_id: sea_orm::ActiveValue::Set(pb.stat.question_id),
                    question_article_live: sea_orm::ActiveValue::Set(pb.stat.question_article_live),
                    question_article_slug: sea_orm::ActiveValue::Set(pb.stat.question_article_slug),
                    question_article_has_video_solution: sea_orm::ActiveValue::Set(pb.stat.question_article_has_video_solution),
                    question_title: sea_orm::ActiveValue::Set(pb.stat.question_title),
                    question_title_slug: sea_orm::ActiveValue::Set(pb.stat.question_title_slug),
                    question_hide: sea_orm::ActiveValue::Set(pb.stat.question_hide),
                    total_acs: sea_orm::ActiveValue::Set(pb.stat.total_acs),
                    total_submitted: sea_orm::ActiveValue::Set(pb.stat.total_submitted),
                    frontend_question_id: sea_orm::ActiveValue::Set(pb.stat.frontend_question_id),
                    is_new_question: sea_orm::ActiveValue::Set(pb.stat.is_new_question),
                    status: sea_orm::ActiveValue::Set(pb.status),
                    difficulty: sea_orm::ActiveValue::Set(pb.difficulty.level),
                    paid_only: sea_orm::ActiveValue::Set(pb.paid_only),
                    is_favor: sea_orm::ActiveValue::Set(pb.is_favor),
                    frequency: sea_orm::ActiveValue::Set(pb.frequency),
                    progress: sea_orm::ActiveValue::Set(pb.progress),
                };

                let res = Problem::update(pb_db)
                    .exec(&db)
                    .await
                    .into_diagnostic()?;
            }
        }

        Ok(())
    }
}
