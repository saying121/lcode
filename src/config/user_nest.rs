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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Urls {
    pub origin: String,
    pub graphql: String,
    pub all_problem_api: String,
    pub submit: String,
    pub test: String,
    pub submissions: String,
    pub favorites: String,
}

impl Default for Urls {
    fn default() -> Self {
        let suffix = "com";
        Urls {
            origin: format!("https://leetcode.{}", suffix),
            graphql: format!("https://leetcode.{}/graphql", suffix),
            all_problem_api: format!(
                "https://leetcode.{}/api/problems/$category",
                suffix
            ),
            submit: format!("https://leetcode.{}/problems/$slug/submit/", suffix),
            test: format!(
                "https://leetcode.{}/problems/$slug/interpret_solution/",
                suffix
            ),
            submissions: format!(
                "https://leetcode.{}/submissions/detail/$id/check/",
                suffix
            ),
            favorites: format!("https://leetcode.{}/list/api/questions", suffix),
        }
    }
}
