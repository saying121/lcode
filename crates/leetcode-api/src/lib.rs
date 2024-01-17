use std::collections::HashMap;

pub mod dao;
pub mod entities;
pub mod leetcode;
#[cfg(feature = "render")]
pub mod render;

pub type Json = HashMap<&'static str, String>;

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
