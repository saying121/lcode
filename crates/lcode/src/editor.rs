use std::process::Command;

use futures::StreamExt;
use lcode_config::config::global::{G_CONFIG_PATH, G_LOG_PATH, G_USER_CONFIG};
use leetcode_api::{dao::{query::Query, save_info}, leetcode::IdSlug};
use miette::{IntoDiagnostic, Result};
use tokio::{
    fs::{self, create_dir_all, OpenOptions},
    io::AsyncWriteExt,
};
use tracing::{debug, instrument};

use crate::glob_leetcode;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub enum CodeTestFile {
    #[default]
    Code,
    Test,
}

pub async fn integr_cargo(id: &str, code_path: &str) -> Result<()> {
    create_dir_all(&G_USER_CONFIG.config.code_dir)
        .await
        .into_diagnostic()?;
    let mut cargo_path = G_USER_CONFIG.config.code_dir.clone();
    cargo_path.push("Cargo.toml");

    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&cargo_path)
        .await
        .into_diagnostic()?;
    let metadata = fs::metadata(&cargo_path)
        .await
        .into_diagnostic()?;
    if metadata.len() == 0 {
        f.write_all(
            r#"[package]
name    = "my-leetcode"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = { version = "0.8.5" }

"#
            .as_bytes(),
        )
        .await
        .into_diagnostic()?;
    }
    let cargo_str = fs::read_to_string(&cargo_path)
        .await
        .into_diagnostic()?;
    let cont = futures::stream::iter(cargo_str.split('\n'))
        .any(|f| async { f.contains(&format!("\"{id}\"")) })
        .await;

    if !cont {
        let append = format!("[[bin]]\nname = \"{}\"\npath = \"./{}\"\n", id, code_path);
        f.write_all(append.as_bytes())
            .await
            .into_diagnostic()?;
    }

    Ok(())
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Editor;

impl Editor {
    #[instrument]
    pub async fn open(idslug: IdSlug, ct: CodeTestFile) -> Result<()> {
        let pb = Query::get_question_index(&idslug).await?;

        let chf = save_info::CacheFile::build(&pb).await?;

        let qs = glob_leetcode()
            .await
            .get_qs_detail(idslug, false)
            .await?;

        if G_USER_CONFIG.config.cargo_integr && G_USER_CONFIG.config.lang.as_str() == "rust" {
            tokio::spawn(async move {
                let pat = format!(
                    "{}_{}/{}.rs",
                    pb.question_id, pb.question_title_slug, pb.question_id
                );
                integr_cargo(&qs.question_id, &pat)
                    .await
                    .ok();
            });
        }

        let mut ed = G_USER_CONFIG.config.editor.clone();
        debug!("get editor: {:#?}", ed);

        let code_path = chf
            .code_path
            .to_string_lossy()
            .to_string();
        let contend_path = chf
            .content_path
            .to_string_lossy()
            .to_string();
        match ct {
            CodeTestFile::Code => {
                let editor = G_USER_CONFIG
                    .config
                    .editor
                    .front()
                    .map_or("vim", |v| v.as_str());

                if editor.contains("vim") {
                    ed.extend([
                        code_path,
                        "-c".to_owned(),
                        format!("vsplit {}", contend_path),
                    ]);
                }
                else if editor == "helix" {
                    ed.extend(["--vsplit".to_owned(), contend_path, code_path]);
                }
                else {
                    ed.push_back(code_path);
                }
            },
            CodeTestFile::Test => {
                ed.push_back(
                    chf.test_case_path
                        .to_string_lossy()
                        .to_string(),
                );
            },
        };

        Command::new(
            ed.pop_front()
                .unwrap_or_else(|| "vim".to_owned()),
        )
        .args(ed)
        .status()
        .into_diagnostic()?;

        Ok(())
    }

    pub fn edit_config() -> Result<()> {
        let mut ed = G_USER_CONFIG.config.editor.clone();
        ed.push_back(
            G_CONFIG_PATH
                .to_string_lossy()
                .to_string(),
        );

        Command::new(
            ed.pop_front()
                .unwrap_or_else(|| "vim".to_owned()),
        )
        .args(ed)
        .status()
        .into_diagnostic()?;

        Ok(())
    }
    pub fn edit_log() -> Result<()> {
        let mut ed = G_USER_CONFIG.config.editor.clone();
        ed.push_back(G_LOG_PATH.to_string_lossy().to_string());

        Command::new(
            ed.pop_front()
                .unwrap_or_else(|| "vim".to_owned()),
        )
        .args(ed)
        .status()
        .into_diagnostic()?;

        Ok(())
    }
}
