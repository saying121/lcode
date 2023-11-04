//! # Brush leetcode under the terminal
//!
//! - 【[中文文档](./README-CN.md)】
//!
//! <!--toc:start-->
//! - [Brush leetcode under the terminal](#brush-leetcode-under-the-terminal)
//!   - [Install](#install)
//!   - [Useage](#useage)
//!   - [Videos](#videos)
//!   - [Configuration](#configuration)
//!     - [Important](#important)
//!     - [Here are the explanations for each field](#here-are-the-explanations-for-each-field)
//!   - [Tui Keymap](#tui-keymap)
//!   - [Fuzzy Search](#fuzzy-search)
//!   - [Database error](#database-error)
//! <!--toc:end-->
//!
//! ## Install
//!
//! **Linux** Option dependencies(a implement `SecretService` service Application)：
//!
//! - `gnome-keyring`
//! - `kwallet`
//! - `KeePassXC`
//! - `libsecret`
//!
//! ---
//!
//! - stable
//!
//! ```shell
//! cargo install lcode
//! ```
//!
//! - nightly
//!
//! ```shell
//! cargo install --git=https://github.com/saying121/leetcode-cn-en-cli.git --force
//! ```
//!
//! ## Useage
//!
//! Generate configuration, manual modification of the configuration is also possible,
//! and it will be automatically generated at runtime.
//! Without -c, it will be generated in English.
//!
//! ```shell
//! lcode gencon -c
//! ```
//!
//! Synchronize basic data first.
//!
//! ```shell
//! lcode sync
//! ```
//!
//! View the documentation for assistance.
//!
//! ```shell
//! lcode -h
//! ```
//!
//! Begin selecting a question.
//!
//! ```shell
//! lcode fzy <edit>
//! ```
//!
//! ## Videos
//!
//! <https://github.com/saying121/leetcode-cn-en-cli/assets/74663483/57a633e5-6bae-4816-a224-d7d61d2141af>
//!
//! <https://github.com/saying121/leetcode-cn-en-cli/assets/74663483/7917a65c-b7a9-4305-b87f-5d2ddc8cb760>
//!
//! ![filter en](./pictures/filter_en.png)
//!
//! ## Configuration
//!
//! The configuration located
//!
//! - Linux: `~/.config/leetcode-cn-en-cli/config.toml`
//! - macos: `~/.config/leetcode-cn-en-cli/config.toml`
//! - Windows: `|C:\Users\Alice\AppData\Roaming`
//!
//! The code default located
//!
//! - Linux: `$HOME/.local/share/leetcode-cn-en-cli/`
//! - macOS: `$HOME/Library/Application Support/leetcode-cn-en-cli`
//! - Windows: `C:\Users\Alice\AppData\Local\leetcode-cn-en-cli`
//!
//! default:
//! ![default](./pictures/screen_shot_.png)
//!
//! ```toml
//! translate = false
//! column = 4
//! num_sublist = 10
//! page_size = 25
//! editor = ["vim"]
//! lang = "rust"
//! code_dir = "/home/user/.local/share/leetcode-cn-en-cli"
//!
//! url_suffix = "com"
//!
//! [support_lang]
//! langs = ["rust", "bash", "c", "cpp", "csharp", "golang", "java", "javascript", "kotlin", "mysql", "php", "python", "python3", "ruby", "scala", "swift", "typescript", "racket", "erlang", "elixir", "dart"]
//!
//! [cookies]
//! csrf = ""
//! session = ""
//! ```
//!
//! ### Important
//!
//! **First, login leetcode in browser for generate cookies**
//!
//! `browser` can fill in `chrome`, `edge`, `firefox`, `librewolf`.
//!
//! Now support this browser, and just test in Linux.(firefox should support three systems)
//! If use the section，be careful not to clear cookies when closing the browser.
//!
//! `[cookies]` section
//!
//! - If the two subfields are not empty,use the content filled by the user.
//!   And not use other method to get cookies。
//!
//!   - Fill in manually：
//!
//!     Press <kbd>F12</kbd> on the browser's `leetcode.com/com` page,
//!     Find the **Cookie** field, copy the **csrftoken=\<content\>;**
//!     and **`LEETCODE_SESSION`=\<content\>;** sections inside it into the configuration.
//!
//! - If user filled `browser` , will try to use the browser to get cookies.
//!
//! - If neither of the above two items is filled in,
//!   and then use this order _firefox_ -> _edge_ -> _chrome_ -> _librewolf_ try to get cookies.
//!
//! ### Here are the explanations for each field
//!
//! Fill in `false` or `true`, default is `false`.
//! If `true` is chosen, the translated content will be used to display the question details.
//!
//! ```toml
//! translate = false
//! ```
//!
//! ---
//!
//! When retrieving the **submissionlist**, how many columns should be displayed.
//!
//! ```toml
//! column = 4
//! ```
//!
//! ---
//!
//! How many recent entries of the submissionlist information should be displayed.
//!
//! ```toml
//! num_sublist = 10
//! ```
//!
//! ---
//!
//! How many questions should be displayed at once when interactively selecting a question.
//!
//! ```toml
//! page_size = 25
//! ```
//!
//! ---
//!
//! Fill in your editor, it will attempt to retrieve it from the environment variables EDITOR and VISUAL,
//! otherwise it will default to vim.
//!
//! ```toml
//! editor = ["vim"]
//! ```
//!
//! You can add additional parameters at the end.
//!
//! ```toml
//! editor = ["vim", "--noplugin"]
//! ```
//!
//! ---
//!
//! Set your selected programming language.
//!
//! ```toml
//! lang = "rust"
//! ```
//!
//! ---
//!
//! Set the location for storing code and test cases.
//!
//! ```toml
//! code_dir = "/home/user/.local/share/leetcode-cn-en-cli"
//! ```
//!
//! You can also write it like this, starting with `~`
//!
//! ```toml
//! code_dir = "~/.local/share/leetcode-cn-en-cli"
//! ```
//!
//! ---
//!
//! Fill in `com` or `cn`, for set `leetcode.com` or `leetcode.cn`.
//!
//! ```toml
//! url_suffix = "com"
//! ```
//!
//! ---
//!
//! ```toml
//! [support_lang.rust]
//! start = "//start/"
//! end = "//end/"
//! inject_start = ""
//! inject_end = "struct Solution;\n\nfn main() {\n    println!(\"{:#?}\", Solution::function());\n}"
//! [support_lang.c]
//! ...
//! ```
//!
//! will from this generate code template.
//!
//! can write multi line,`"""..."""` or `'''...'''`：
//!
//! ```toml
//! inject_end = """struct Solution;
//! fn main() {
//!     println!("{:#?}", Solution::function());
//! }"""
//! ```
//!
//! Example: 108
//!
//! ```rust
//! // Definition for a binary tree node.
//! #[derive(Debug, PartialEq, Eq)]
//! pub struct TreeNode {
//!     pub val: i32,
//!     pub left: Option<Rc<RefCell<TreeNode>>>,
//!     pub right: Option<Rc<RefCell<TreeNode>>>,
//! }
//!
//! impl TreeNode {
//!     #[inline]
//!     pub fn new(val: i32) -> Self {
//!         TreeNode {
//!             val,
//!             left: None,
//!             right: None,
//!         }
//!     }
//! }
//!
//! struct Solution;
//!
//! //start/
//! // ...something
//! use std::cell::RefCell;
//! use std::rc::Rc;
//! impl Solution {
//!     pub fn sorted_array_to_bst(mut nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
//!         let len = nums.len();
//!         if len == 0 {
//!             return None;
//!         }
//!         let root = Rc::new(RefCell::new(TreeNode::new(nums[len / 2])));
//!         let mut right = nums.split_off(len / 2);
//!         right.remove(0);
//!         root.borrow_mut().left = Self::sorted_array_to_bst(nums);
//!         root.borrow_mut().right = Self::sorted_array_to_bst(right);
//!
//!         Some(root)
//!     }
//! }
//! //end/
//!
//! println!(
//!     "{:#?}",
//!     Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
//! );
//! ```
//!
//! When submit to leetcode, only content between `support_lang.rust.start`
//! and `support_lang.rust.start` will be uploaded.
//!
//! If don't have this will uploaded all content.
//!
//! ## Tui Keymap
//!
//! |              key               |     global     |
//! | :----------------------------: | :------------: |
//! | <kbd>Shift-Tab/⬅/➡/Tab</kbd> | prev/next tab  |
//! |       <kbd>Ctrl-l</kbd>        | refresh screen |
//! |       <kbd>Ctrl-q</kbd>        |      exit      |
//!
//! |       key        |        tab0/select        |
//! | :--------------: | :-----------------------: |
//! |  <kbd>j/k</kbd>  |     down/up question      |
//! | <kbd>gg/G</kbd>  |        first/last         |
//! |   <kbd>o</kbd>   |   open with your editor   |
//! | <kbd>Enter</kbd> |      go to edit tab       |
//! |   <kbd>S</kbd>   | sync question information |
//!
//! |        key        |             tab1/edit              |
//! | :---------------: | :--------------------------------: |
//! |  <kbd>j/k</kbd>   |          scroll question           |
//! |  <kbd>gg/G</kbd>  |      question content top/end      |
//! | <kbd>ctrl-p</kbd> |         toggle submit menu         |
//! | <kbd>ctrl-s</kbd> |        toggle submit result        |
//! | <kbd>ctrl-t</kbd> |         toggle test result         |
//! |   <kbd>S</kbd>    | Submit code(just show submit menu) |
//! |   <kbd>T</kbd>    |  Test code(just show submit menu)  |
//!
//! Please check the Tui interface for specific keymap information.
//!
//! ## Fuzzy Search
//!
//! fuzzy search tui and cli implement is same，in cli paid only is true ，in tui also can input `true`/`P.O.: tru` for filter.
//!
//! ## Database error
//!
//! Since `leetcode.cn` and `leetcode.com` obtain different information, database errors may occur after switching between *cn* and *com*.
//!
//! At this time, please re-synchronize the data `lcode sync -f` (recommand), or press <kbd>S</kbd> on a tab in the Tui interface.

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
