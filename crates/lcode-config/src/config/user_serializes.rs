use std::{collections::VecDeque, env, path::PathBuf};

use serde::{Deserialize, Deserializer, Serializer};

use super::{global::G_APP_NAME, user_nest::Suffix};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Suffix, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let res = match s.as_bytes() {
        b"cn" => Suffix::Cn,
        _ => Suffix::Com,
    };
    Ok(res)
}
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize<S>(v: &Suffix, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let res = match v {
        Suffix::Cn => "cn",
        Suffix::Com => "com",
    };
    serializer.serialize_str(res)
}

pub(super) const fn cargo_default() -> bool {
    true
}
pub(super) fn lang_default() -> String {
    "rust".to_owned()
}

/// "~/.local/share/leetcode-cn-en-cli"
pub(super) fn default_code_dir() -> PathBuf {
    let mut code_dir = dirs::data_local_dir().expect("new data local dir failed");
    code_dir.push(G_APP_NAME);
    code_dir
}
/// Get user's editor from environment variable EDITOR and VISUAL
pub(super) fn default_editor() -> VecDeque<String> {
    let editor = env::var("EDITOR")
        .unwrap_or_else(|_| env::var("VISUAL").unwrap_or_else(|_| "vim".to_owned()));
    VecDeque::from([editor])
}
