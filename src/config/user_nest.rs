use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SupportLang {
    pub(crate) langs: Vec<String>,
}

impl Default for SupportLang {
    fn default() -> Self {
        Self {
            langs: vec![
                "rust".to_owned(),
                "bash".to_owned(),
                "c".to_owned(),
                "cpp".to_owned(),
                "csharp".to_owned(),
                "golang".to_owned(),
                "java".to_owned(),
                "javascript".to_owned(),
                "kotlin".to_owned(),
                "mysql".to_owned(),
                "php".to_owned(),
                "python".to_owned(),
                "python3".to_owned(),
                "ruby".to_owned(),
                "scala".to_owned(),
                "swift".to_owned(),
                "typescript".to_owned(),
                "racket".to_owned(),
                "erlang".to_owned(),
                "elixir".to_owned(),
                "dart".to_owned(),
            ],
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cookies {
    pub csrf: String,
    pub session: String,
}

impl ToString for Cookies {
    fn to_string(&self) -> String {
        format!("LEETCODE_SESSION={};csrftoken={};", self.session, self.csrf)
    }
}
