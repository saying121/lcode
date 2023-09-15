use std::path::PathBuf;

use miette::{IntoDiagnostic, Result};
use tokio::{
    fs::{create_dir_all, OpenOptions},
    io::AsyncWriteExt,
    task::spawn_blocking,
};
use tracing::{instrument, trace};

use crate::{
    config::{
        global::{glob_code_dir, glob_user_config},
        User,
    },
    dao::get_question_index_exact,
    entities::*,
    leetcode::{qs_detail::Question, IdSlug},
    render::Render,
};

/// write info to file
pub struct CacheFile {
    pub code_path: PathBuf,
    pub test_case_path: PathBuf,
    pub content_path: PathBuf,
}

impl CacheFile {
    /// Get code, test, content dir
    #[instrument]
    pub async fn new(idslug: &IdSlug) -> Result<Self> {
        let pb: index::Model = get_question_index_exact(idslug).await?;
        let user_config = spawn_blocking(glob_user_config)
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
        let temp = match glob_user_config().translate {
            true => "cn",
            false => "en",
        };
        let detail_file_name = format!("{}_detail_{}.md", pb.question_id, temp);
        content_path.push(detail_file_name);
        trace!("content case path: {:?}", content_path);
        Ok(Self {
            content_path,
            code_path,
            test_case_path,
        })
    }
    /// Write a question's code and test case to file
    #[instrument(skip(self, detail, user))]
    pub async fn write_to_file(&self, detail: Question, user: &User) -> Result<()> {
        let content = detail.to_md_str();
        let (r1, r2) = tokio::join!(
            Self::write_file(&self.test_case_path, &detail.example_testcases),
            Self::write_file(&self.content_path, &content)
        );
        r1?;
        r2?;

        for code_snippet in &detail.code_snippets {
            if code_snippet.lang_slug == user.lang {
                Self::write_file(&self.code_path, &code_snippet.code).await?;
            }
        }

        // if this question not support this lang
        if !self.code_path.exists() {
            let mut temp =
                "this question not support the lang or is paid only\n\nsupport below:\n"
                    .to_string();

            for code_snippet in &detail.code_snippets {
                temp += &format!("{}\n", code_snippet.lang_slug);
            }
            Self::write_file(&self.code_path, &temp).await?;
        }

        Ok(())
    }

    async fn write_file(path: &PathBuf, val: &str) -> Result<()> {
        if !path.exists() {
            create_dir_all(
                &path
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
                .open(&path)
                .await
                .into_diagnostic()?;
            file.write_all(val.as_bytes())
                .await
                .into_diagnostic()?;

            file.sync_all()
                .await
                .into_diagnostic()?;
        }
        Ok(())
    }
}
