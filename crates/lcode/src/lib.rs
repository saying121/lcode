// #![doc = include_str!("./README.md")]

mod app;
pub mod cli;
pub mod editor;
pub mod fuzzy_search;
pub mod mytui;
pub mod panic_hook;

pub fn star() {
    open::that_detached("https://github.com/saying121/leetcode-cn-en-cli").unwrap_or_default();
}

pub use leetcode_api::glob_leetcode;
