pub mod read_config;
pub mod user_nested;
mod user_serializes;

use std::{collections::VecDeque, path::PathBuf};

use decrypt_cookies::browser::cookies::LeetCodeCookies;
use serde::{Deserialize, Serialize};
use user_nested::*;

use self::user_serializes::*;
use crate::{global, keymap::TuiKeyMap};

/// config for user
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct LcodeConfig {
    #[serde(skip)]
    pub urls: Urls,
    #[serde(default)]
    pub config: Config,
    #[serde(default)]
    pub cookies: LeetCodeCookies,
    #[serde(default)]
    pub langs: SupportLang,
    #[serde(default)]
    pub keymap: TuiKeyMap,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub translate: bool,
    #[serde(default, with = "user_serializes")]
    pub url_suffix: Suffix,
    #[serde(default)]
    pub column: usize,
    #[serde(default)]
    pub num_sublist: u32,
    #[serde(default)]
    pub page_size: usize,
    #[serde(default = "default_editor")]
    pub editor: VecDeque<String>,
    #[serde(default = "lang_default")]
    pub lang: String,
    #[serde(default = "default_code_dir")]
    pub code_dir: PathBuf,
    #[serde(default)]
    pub browser: String,
    #[serde(default = "default_true")]
    pub cargo_integr: bool,
    #[serde(default)]
    /// create qs dir use frontend id
    pub dir_with_frontend_id: bool,
    #[serde(default)]
    pub show_avatar: bool,
}

impl Config {
    pub fn new(suffix: Suffix) -> Self {
        let (url_suffix, translate) = match suffix {
            Suffix::Cn => (Suffix::Cn, true),
            Suffix::Com => (Suffix::Com, false),
        };
        Self {
            translate,
            url_suffix,
            ..Default::default()
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            translate: false,
            column: 4,
            num_sublist: 16,
            page_size: 25,
            url_suffix: Suffix::default(),
            editor: default_editor(),
            lang: "rust".to_owned(),
            code_dir: default_code_dir(),
            browser: String::new(),
            cargo_integr: true,
            dir_with_frontend_id: false,
            show_avatar: false,
        }
    }
}

impl LcodeConfig {
    ///  "cn"  "en"
    pub fn new(tongue: Suffix) -> Self {
        let config = Config::new(tongue);
        Self { config, ..Default::default() }
    }

    /// `start`, `end`, `inject_end`, `inject_end`
    pub fn get_lang_info(&self) -> (String, String, String, String) {
        macro_rules! return_info_macro {
            ($($struct_name:ident),*) => {
                match self.config.lang.as_str() {
                    $(
                        stringify!($struct_name) => self.langs.$struct_name.return_info(),
                    )*
                    _ => self.langs.rust.return_info(),
                }
            };
        }

        return_info_macro!(
            rust, bash, c, cpp, csharp, golang, java, javascript, kotlin, mysql, php, python,
            python3, ruby, scala, swift, typescript, racket, erlang, elixir, dart
        )
    }

    /// get code file suffix
    pub fn get_suffix(&self) -> &str {
        let sp_lang = &global::G_SUPPORT_LANGS;
        sp_lang
            .get(self.config.lang.as_str())
            .copied()
            .unwrap_or_default()
    }
}
