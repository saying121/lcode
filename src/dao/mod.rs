pub mod query_qs;

use std::path::PathBuf;

use crate::entities::prelude::*;
use crate::{
    config::{
        global::{glob_code_dir, glob_user_config, self},
        User,
    },
    dao::query_qs::get_question_index_exact,
    entities::*,
    leetcode::{qs_detail::Question, IdSlug},
    render::Render,
};
use miette::{IntoDiagnostic, Result, Error};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};
use tokio::join;
use tokio::{
    fs::{create_dir_all, OpenOptions},
    io::AsyncWriteExt,
    task::spawn_blocking,
};
use tracing::{debug, instrument, trace};

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

/// get all problem's base info
///
/// * `client`: reqwest client
/// * `headers`: reqwest headers
///
/// # Example
/// ```rust
/// Cache::new.await?.get_all_problems(false).await?
/// ```
#[derive(Debug)]
pub struct CacheFile;

impl CacheFile {
    /// Write a question's code and test case to file
    #[instrument(skip(detail, user))]
    pub async fn write_to_file(detail: Question, user: &User) -> Result<()> {
        let (code_path, test_file_path, content_path) =
            Self::get_code_and_test_path(IdSlug::Id(
                detail
                    .question_id
                    .parse()
                    .into_diagnostic()?,
            ))
            .await?;
        debug!("test file path: {:?}", test_file_path);

        if !test_file_path.exists() {
            create_dir_all(
                &test_file_path
                    .parent()
                    .unwrap_or_else(|| glob_code_dir()),
            )
            .await
            .into_diagnostic()?;
            debug!("example_testcases: {}", detail.example_testcases);
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&test_file_path)
                .await
                .into_diagnostic()?;
            file.write_all(detail.example_testcases.as_bytes())
                .await
                .into_diagnostic()?;
            file.sync_all()
                .await
                .into_diagnostic()?;
        }

        for code_snippet in &detail.code_snippets {
            if code_snippet.lang_slug == user.lang {
                if !code_path.exists() {
                    create_dir_all(
                        &code_path
                            .parent()
                            .unwrap_or_else(|| glob_code_dir()),
                    )
                    .await
                    .into_diagnostic()
                    .unwrap();

                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(&code_path)
                        .await
                        .into_diagnostic()?;
                    file.write_all(code_snippet.code.as_bytes())
                        .await
                        .into_diagnostic()?;
                    file.sync_all()
                        .await
                        .into_diagnostic()?;
                }
            }
        }
        // if this question not support this lang
        if !code_path.exists() {
            create_dir_all(
                &code_path
                    .parent()
                    .unwrap_or_else(|| glob_code_dir()),
            )
            .await
            .into_diagnostic()
            .unwrap();

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&code_path)
                .await
                .into_diagnostic()?;
            file.write_all(
                "this question not support the lang\n\nsupport below:\n".as_bytes(),
            )
            .await
            .into_diagnostic()?;
            for code_snippet in &detail.code_snippets {
                file.write_all(format!("{}\n", code_snippet.lang_slug).as_bytes())
                    .await
                    .into_diagnostic()?;
            }

            file.sync_all()
                .await
                .into_diagnostic()?;
        }
        if !content_path.exists() {
            create_dir_all(
                &content_path
                    .parent()
                    .unwrap_or_else(|| glob_code_dir()),
            )
            .await
            .into_diagnostic()
            .unwrap();

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&content_path)
                .await
                .into_diagnostic()?;
            file.write_all(detail.to_md_str().as_bytes())
                .await
                .into_diagnostic()?;

            file.sync_all()
                .await
                .into_diagnostic()?;
        }
        Ok(())
    }

    /// Get code and test case dir
    /// (code, test)
    #[instrument(skip())]
    pub async fn get_code_and_test_path(
        idslug: IdSlug,
    ) -> Result<(PathBuf, PathBuf, PathBuf)> {
        let pb: index::Model = get_question_index_exact(idslug).await?;
        let user_config = spawn_blocking(|| glob_user_config())
            .await
            .into_diagnostic()?;
        let mut cache_path = user_config.code_dir.to_owned();
        let sub_dir = format!("{}_{}", pb.question_id, pb.question_title_slug,);
        cache_path.push(sub_dir);
        create_dir_all(&cache_path)
            .await
            .into_diagnostic()?;

        let mut code_path = cache_path.to_owned();
        let code_file_name = format!("{}{}", pb.question_id, user_config.get_suffix());
        code_path.push(code_file_name);
        trace!("code path: {:?}", code_path);

        let mut test_case_path = cache_path.to_owned();
        let test_file_name = format!("{}_test_case.txt", pb.question_id);
        test_case_path.push(test_file_name);
        trace!("test case path: {:?}", test_case_path);

        let mut content_path = cache_path.to_owned();
        let detail_file_name = format!("detail.md");
        content_path.push(detail_file_name);
        trace!("content case path: {:?}", content_path);
        Ok((code_path, test_case_path, content_path))
    }
}
