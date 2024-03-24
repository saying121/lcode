pub mod qs_detail;
pub mod run_res;
pub mod submit_list;

use std::{
    io::prelude::Write,
    process::{Command, Stdio},
};

#[cfg(feature = "ratatui")]
use ratatui::text::Line;
use regex::{Captures, Regex};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub enum SupSub {
    #[default]
    Sup,
    Sub,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub enum StTy {
    Str,
    #[default]
    Tty,
}

pub trait Render {
    /// uniform treatment `Question` detail to markdown String
    ///
    /// * `_with_env`: for `Question` whether display `Question` Compile Environment
    fn to_md_str(&self, _with_env: bool) -> String {
        String::new()
    }

    /// for ratatui's paragraph widget
    #[cfg(feature = "ratatui")]
    fn to_tui_vec(&self) -> Vec<Line>;

    /// use [`mdcat`](https://github.com/swsnr/mdcat/) render question content
    fn render_with_mdcat(&self) {
        let content = self.to_md_str(false);
        'out: {
            let Ok(mut child) = Command::new("mdcat")
                .stdin(Stdio::piped())
                .stdout(Stdio::inherit())
                .spawn()
            else {
                break 'out;
            };
            if let Some(mut stdin) = child.stdin.take() {
                if stdin
                    .write_all(content.as_bytes())
                    .is_err()
                {
                    break 'out;
                };
                // stdin drop here
            }
            else {
                break 'out;
            };

            let Ok(exit_status) = child.wait()
            else {
                break 'out;
            };
            if exit_status.success() {
                return;
            }
        }

        println!("{content}");
    }
}

pub fn to_sub_sup_script(content: &str) -> String {
    let sup_re = Regex::new("<sup>(?P<num>[0-9]*)</sup>").expect("regex new failed");
    let sub_re = Regex::new("<sub>(?P<num>[0-9]*)</sub>").expect("regex new failed");

    let content = sup_re.replace_all(content, |cap: &Captures| {
        let num = cap["num"].parse().unwrap_or_default();
        superscript(num, SupSub::Sup)
    });

    let content = sub_re.replace_all(&content, |cap: &Captures| {
        let num = cap["num"].parse().unwrap_or_default();
        superscript(num, SupSub::Sub)
    });

    content.to_string()
}

const SUPER_NUM: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
const SUB_NUM: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

pub fn superscript(n: usize, sub_or_sup: SupSub) -> String {
    let sub_or_sup = match sub_or_sup {
        SupSub::Sup => SUPER_NUM,
        SupSub::Sub => SUB_NUM,
    };
    match n {
        0..=9 => sub_or_sup[n].to_string(),
        mut num => {
            // 2 is enough, avoid frequently create string
            let mut res = String::with_capacity(2);
            while num > 0 {
                res.push(sub_or_sup[num % 10]);
                num /= 10;
            }
            res.chars().rev().collect::<String>()
        },
    }
}
