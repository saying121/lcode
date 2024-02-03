use std::{ops::Not, path::PathBuf};

use lcode_config::config::global::G_USER_CONFIG;
use miette::{IntoDiagnostic, Result};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::{instrument, trace};

use crate::{
    entities::*,
    leetcode::{question::qs_detail::Question, IdSlug},
    render::Render,
};

/// write info to file
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct CacheFile {
    pub code_path:      PathBuf,
    pub test_case_path: PathBuf,
    pub content_path:   PathBuf,
}

impl CacheFile {
    /// Get code, test, content dir
    #[instrument]
    pub async fn build(pb: &index::Model) -> Result<Self> {
        let mut cache_path = G_USER_CONFIG.config.code_dir.clone();
        let sub_dir = format!("{}_{}", pb.question_id, pb.question_title_slug);
        cache_path.push(sub_dir);

        create_dir_all(&cache_path)
            .await
            .into_diagnostic()?;

        let mut code_path = cache_path.clone();
        let code_file_name = format!("{}{}", pb.question_id, G_USER_CONFIG.get_suffix());
        code_path.push(code_file_name);
        trace!("code path: {:?}", code_path);

        let mut test_case_path = cache_path.clone();
        let test_file_name = format!("{}_test_case.txt", pb.question_id);
        test_case_path.push(test_file_name);
        trace!("test case path: {:?}", test_case_path);

        let mut content_path = cache_path.clone();
        let temp = if G_USER_CONFIG.config.translate {
            "cn"
        }
        else {
            "en"
        };
        let detail_file_name = format!("{}_detail_{}.md", pb.question_id, temp);
        content_path.push(detail_file_name);
        trace!("content case path: {:?}", content_path);
        Ok(Self {
            code_path,
            test_case_path,
            content_path,
        })
    }
    /// Write a question's `content`, `code` and `test_case` to file
    pub async fn write_to_file(&self, detail: &Question) -> Result<()> {
        let content = detail.to_md_str(true);

        let (r1, r2) = tokio::join!(
            Self::write_file(&self.test_case_path, &detail.example_testcases),
            Self::write_file(&self.content_path, &content)
        );
        r1?;
        r2?;

        if let Some(snippets) = &detail.code_snippets {
            for snippet in snippets {
                if snippet.lang_slug == G_USER_CONFIG.config.lang {
                    let (start, end, mut inject_start, inject_end) = G_USER_CONFIG.get_lang_info();

                    if !inject_start.is_empty() {
                        inject_start += "\n";
                    }
                    let code_str = format!(
                        "{}{}\n{}\n{}\n{}",
                        inject_start, start, snippet.code, end, inject_end
                    );
                    Self::write_file(&self.code_path, &code_str).await?;
                }
            }
        }

        // if this question not support this lang, or is paid only
        if !self.code_path.exists() {
            let temp = if detail.is_paid_only {
                "this question is paid only".to_owned()
            }
            else {
                let mut temp = format!(
                    "this question not support {} \n\nsupport below:\n",
                    G_USER_CONFIG.config.lang
                );
                if let Some(snippets) = &detail.code_snippets {
                    for snippet in snippets {
                        temp += &format!("{}\n", snippet.lang_slug);
                    }
                }
                temp
            };

            Self::write_file(&self.code_path, &temp).await?;
        }

        Ok(())
    }
    pub async fn get_user_code(&self, idslug: &IdSlug) -> Result<(String, String)> {
        let (code_file, test_case_file) = tokio::join!(
            File::open(&self.code_path),
            File::open(&self.test_case_path)
        );

        let (mut code_file, mut test_case_file) = (
            code_file.map_err(|err| {
                miette::miette!(
                    "Error: {}. There is no code file, maybe you changed the name, please get \
                     **{}** question detail again",
                    err,
                    idslug
                )
            })?,
            test_case_file.map_err(|err| {
                miette::miette!(
                    "Error: {}. There is no test case file, maybe you changed the name, please \
                     remove relate file and get **{}** question detail again, or manual create a \
                     same name blank file",
                    err,
                    idslug
                )
            })?,
        );

        let mut code = String::new();
        let mut test_case = String::new();

        let (code_res, test_case_res) = tokio::join!(
            code_file.read_to_string(&mut code),
            test_case_file.read_to_string(&mut test_case)
        );
        code_res.into_diagnostic()?;
        test_case_res.into_diagnostic()?;

        Ok((code, test_case))
    }

    /// if file not exists, create file and write something
    async fn write_file(path: &PathBuf, val: &str) -> Result<()> {
        if path.exists().not() {
            create_dir_all(&path.parent().expect("get path parent failed"))
                .await
                .into_diagnostic()
                .expect("create_dir_all failed");

            let mut file = OpenOptions::new()
                .create(true)
                .truncate(true)
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
