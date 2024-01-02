#![doc = include_str!("../README.md")]
#![allow(unknown_lints)]
#![allow(renamed_and_removed_lints)]

use std::collections::HashMap;

pub mod cli;
pub mod dao;
pub mod editor;
pub mod entities;
pub mod fuzzy_search;
pub mod leetcode;
pub mod mytui;
pub mod panic_hook;
pub mod render;

pub type Json = HashMap<&'static str, String>;

pub fn star() {
    open::that_detached("https://github.com/saying121/leetcode-cn-en-cli").unwrap_or_default();
}

use leetcode::LeetCode;
use tokio::sync::OnceCell;
pub static LEETCODE: OnceCell<LeetCode> = OnceCell::const_new();
/// global leetocde
pub async fn glob_leetcode() -> &'static LeetCode {
    LEETCODE
        .get_or_init(|| async {
            LeetCode::build()
                .await
                .expect("new `LeetCode` failed")
        })
        .await
}
