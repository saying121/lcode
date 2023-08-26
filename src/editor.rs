use crate::{
    config::global::{global_leetcode, global_user_config},
    leetcode::IdSlug,
    storage::Cache,
};
use miette::{IntoDiagnostic, Result};
use tokio::task::spawn_blocking;
use tracing::{debug, instrument};

#[derive(Debug)]
pub enum CodeTestFile {
    Code,
    Test,
}

#[instrument]
pub async fn edit(idslug: IdSlug, cdts: CodeTestFile) -> Result<()> {
    let user = spawn_blocking(|| global_user_config().to_owned())
        .await
        .into_diagnostic()?;
    let (code, test) = Cache::get_code_and_test_path(idslug.clone()).await?;

    if !code.exists() || !test.exists() {
        let leetcode = global_leetcode();
        leetcode
            .get_qs_detail(idslug, false)
            .await?;
    }

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
