use miette::Result;

use crate::chromium::CHROME_STORAGE_NAME;

pub async fn get_pass() -> Result<Vec<u8>> {
    let Ok(var) = std::process::Command::new("security")
        .args([
            "-q",
            "find-generic-password",
            "-wa",
            "Chrome",
            "-s",
            CHROME_STORAGE_NAME,
        ])
        .output()
    else {
        // may not work
        return Ok(b"peanuts".to_vec());
    };

    Ok(var.stdout)
}
