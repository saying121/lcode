use std::process::Command;

use lcode_config::config::global::{CONFIG_PATH, USER_CONFIG};
use miette::{IntoDiagnostic, Result};
use tracing::{debug, instrument};

use crate::{dao::save_info, glob_leetcode, leetcode::IdSlug};

#[derive(Debug)]
pub enum CodeTestFile {
    Code,
    Test,
}

#[instrument]
pub async fn edit(idslug: IdSlug, cdts: CodeTestFile) -> Result<()> {
    let chf = save_info::CacheFile::new(&idslug).await?;

    glob_leetcode()
        .await
        .get_qs_detail(idslug, false)
        .await?;

    let mut ed = USER_CONFIG.config.editor.clone();
    debug!("get editor: {:#?}", ed);

    match cdts {
        CodeTestFile::Code => {
            ed.push_back(
                chf.code_path
                    .to_string_lossy()
                    .to_string(),
            );
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

#[instrument]
pub async fn edit_config() -> Result<()> {
    let mut ed = USER_CONFIG.config.editor.clone();
    ed.push_back(
        CONFIG_PATH
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
