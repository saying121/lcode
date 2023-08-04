use miette::{IntoDiagnostic, Result};
use tracing::{debug, instrument};

use crate::{config::read_config::get_user_conf, leetcode::IdSlug, storage::Cache};

#[derive(Debug)]
pub enum CodeTestFile {
    Code,
    Test,
}

#[instrument]
pub async fn edit(idslug: IdSlug, cdts: CodeTestFile) -> Result<()> {
    let user = get_user_conf().await?;
    let (code, test) = Cache::get_code_and_test_path(idslug, &user).await?;
    let mut ed = user.editor;
    debug!("get editor: {:#?}", ed);

    match cdts {
        CodeTestFile::Code => {
            ed.push_back(code.to_string_lossy().to_string());
        }
        CodeTestFile::Test => {
            ed.push_back(test.to_string_lossy().to_string());
        }
    };

    std::process::Command::new(
        ed.pop_front()
            .unwrap_or("vim".to_string()),
    )
    .args(ed)
    .status()
    .into_diagnostic()?;

    Ok(())
}
