#![doc = include_str!("../README.md")]
#![allow(unknown_lints)]
#![allow(renamed_and_removed_lints)]

pub mod cli;
pub mod editor;
pub mod fuzzy_search;
pub mod mytui;
pub mod panic_hook;
mod app;

pub fn star() {
    open::that_detached("https://github.com/saying121/leetcode-cn-en-cli").unwrap_or_default();
}

pub use leetcode_api::glob_leetcode;
