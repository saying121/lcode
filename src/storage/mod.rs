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
    fs::{create_dir_all, OpenOptions},
    io::AsyncWriteExt,
};
use tracing::{instrument, trace};

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
    #[instrument]
    /// write code and test case to file
    pub async fn write_to_file(detail: Question, user: &User) -> Result<()> {
        trace!("detail:{:?}", detail);

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

        for code_snippet in &detail.code_snippets {
            if code_snippet.lang_slug == user.lang {
                trace!("code_dir: {:#?}", code_dir);

                if !code_dir.exists() {
                    create_dir_all(&code_dir.parent().unwrap())
                        .await
                        .into_diagnostic()
                        .unwrap();
                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(&code_dir)
                        .await
                        .unwrap();
                    file.write_all(&code_snippet.code.as_bytes())
                        .await
                        .unwrap()
                }

                if !test_file_path.exists() {
                    create_dir_all(&test_file_path.parent().unwrap())
                        .await
                        .into_diagnostic()?;
                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(&test_file_path)
                        .await
                        .into_diagnostic()?;
                    file.write_all(&detail.example_testcases.as_bytes())
                        .await
                        .into_diagnostic()?;
                }
            }
        }
        Ok(())
    }

    /// Get code and test case dir
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
