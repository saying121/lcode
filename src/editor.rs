use crate::{
    config::global::{glob_leetcode, glob_user_config},
    dao::save_info,
    leetcode::IdSlug,
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
    let user = spawn_blocking(|| glob_user_config().to_owned())
        .await
        .into_diagnostic()?;
    let chf = save_info::CacheFile::new(&idslug).await?;

    if !chf.code_path.exists() || !chf.test_case_path.exists() {
        let leetcode = glob_leetcode();
        leetcode
            .get_qs_detail(idslug, false)
            .await?;
    }

    let mut ed = user.editor;
    debug!("get editor: {:#?}", ed);

    match cdts {
        CodeTestFile::Code => {
            ed.push_back(
                chf.code_path
                    .to_string_lossy()
                    .to_string(),
            );
        }
        CodeTestFile::Test => {
            ed.push_back(
                chf.test_case_path
                    .to_string_lossy()
                    .to_string(),
            );
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
