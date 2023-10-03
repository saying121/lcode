#![allow(unknown_lints)]

pub mod cli;
pub mod config;
pub mod cookies;
pub mod dao;
pub mod editor;
pub mod entities;
pub mod fuzzy_search;
pub mod leetcode;
pub mod mytui;
pub mod panic_hook;
pub mod render;


pub fn star() {
    open::that_detached("https://github.com/saying121/leetcode-cn-en-cli")
        .unwrap_or_default();
}
