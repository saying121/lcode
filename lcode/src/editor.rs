use std::process::Command;

use miette::{IntoDiagnostic, Result};
use tracing::{debug, instrument};

use crate::{
    config::global::{glob_config, glob_config_path, glob_leetcode},
    dao::save_info,
    leetcode::IdSlug,
};

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

    let mut ed = glob_config().config.editor.clone();
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
    let user = glob_config();

    let mut ed = user.config.editor.clone();
    ed.push_back(
        glob_config_path()
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
