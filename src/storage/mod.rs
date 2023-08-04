pub mod query_question;

use crate::{
    config::User,
    entities::*,
    leetcode::{question_detail::Question, IdSlug},
    storage::query_question::get_question_index_exact,
};
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;
use tokio::{
    fs::{create_dir_all, write, OpenOptions},
    io::AsyncWriteExt,
};
use tracing::{debug, instrument, trace};

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
pub struct Cache;

impl Cache {
    /// write code and test case to file
    #[instrument(skip(detail, user))]
    pub async fn write_to_file(detail: Question, user: &User) -> Result<()> {
        let (code_dir, test_file_path) = Self::get_code_and_test_path(
            IdSlug::Id(
                detail
                    .question_id
                    .parse()
                    .into_diagnostic()?,
            ),
            user,
        )
        .await?;
        debug!("test file path: {:?}", test_file_path);

        if !test_file_path.exists() {
            create_dir_all(&test_file_path.parent().unwrap())
                .await
                .into_diagnostic()?;
            debug!("example_testcases: {}", detail.example_testcases);
            write(test_file_path, detail.example_testcases)
                .await
                .into_diagnostic()?;
        }

        for code_snippet in &detail.code_snippets {
            if code_snippet.lang_slug == user.lang {
                if !code_dir.exists() {
                    create_dir_all(&code_dir.parent().unwrap())
                        .await
                        .into_diagnostic()
                        .unwrap();
                    write(&code_dir, &code_snippet.code)
                        .await
                        .into_diagnostic()?;
                }
            }
        }
        Ok(())
    }

    pub async fn write_test_case(
        test_file_path: PathBuf,
        detail: Question,
    ) -> Result<()> {
        if !test_file_path.exists() {
            create_dir_all(&test_file_path.parent().unwrap())
                .await
                .into_diagnostic()?;
            let mut ts_f = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&test_file_path)
                .await
                .into_diagnostic()?;
            debug!("example_testcases: {}", detail.example_testcases);
            ts_f.write_all(detail.example_testcases.as_bytes())
                .await
                .into_diagnostic()?;
        }
        Ok(())
    }

    /// Get code and test case dir
    #[instrument(skip(user_config))]
    pub async fn get_code_and_test_path(
        idslug: IdSlug,
        user_config: &User,
    ) -> Result<(PathBuf, PathBuf)> {
        let pb: index::Model = get_question_index_exact(idslug).await?;

        let mut code_path = user_config.code_dir.to_owned();
        let code_file_name = format!(
            "{}_{}{}",
            pb.question_id,
            pb.question_title_slug,
            user_config.get_suffix()
        );
        code_path.push(code_file_name);
        trace!("code path: {:?}", code_path);

        let mut test_case_path = user_config.code_dir.to_owned();
        let test_file_name =
            format!("{}_{}{}", pb.question_id, pb.question_title_slug, ".dat");
        test_case_path.push(test_file_name);
        trace!("test case path: {:?}", test_case_path);

        Ok((code_path, test_case_path))
    }
}
