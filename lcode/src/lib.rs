#![doc = include_str!("../README.md")]
#![allow(unknown_lints)]

use std::collections::HashMap;

pub mod cli;
pub mod config;
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
    open::that_detached("https://github.com/saying121/leetcode-cn-en-cli")
        .unwrap_or_default();
}
