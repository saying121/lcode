use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SupportLang {
    pub rust: Rust,
    pub bash: Bash,
    pub c: C,
    pub cpp: Cpp,
    pub csharp: Csharp,
    pub golang: Golang,
    pub java: Java,
    pub javascript: Javascript,
    pub kotlin: Kotlin,
    pub mysql: Mysql,
    pub php: Php,
    pub python: Python,
    pub python3: Python3,
    pub ruby: Ruby,
    pub scala: Scala,
    pub swift: Swift,
    pub typescript: Typescript,
    pub racket: Racket,
    pub erlang: Erlang,
    pub elixir: Elixir,
    pub dart: Dart,
}
impl Rust {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Bash {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl C {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Cpp {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Csharp {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Golang {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Java {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Javascript {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Kotlin {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Mysql {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Php {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Python3 {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Python {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Ruby {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Scala {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Swift {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Typescript {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Erlang {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Elixir {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Racket {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}
impl Dart {
    pub fn return_info(&self) -> (String, String, String, String) {
        (
            self.start.to_owned(),
            self.end.to_owned(),
            self.inject_start.to_owned(),
            self.inject_end.to_owned(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rust {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}

impl Default for Rust {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: "struct Solution;\n".to_owned(),
            inject_end: r#"
fn main() {
    println!("{:#?}", Solution::function());
}"#
            .to_owned(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bash {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Bash {
    fn default() -> Self {
        Self {
            start: "##start#".to_owned(),
            end: "##start#".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct C {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for C {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cpp {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Cpp {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Csharp {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Csharp {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Golang {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Golang {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Java {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Java {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Javascript {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Javascript {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Kotlin {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Kotlin {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mysql {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Mysql {
    fn default() -> Self {
        Self {
            start: "--start-".to_owned(),
            end: "--end-".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Php {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Php {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Python {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Python {
    fn default() -> Self {
        Self {
            start: "##start#".to_owned(),
            end: "##end#".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Python3 {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Python3 {
    fn default() -> Self {
        Self {
            start: "##start#".to_owned(),
            end: "##end#".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ruby {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Ruby {
    fn default() -> Self {
        Self {
            start: "##start#".to_owned(),
            end: "##end#".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scala {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Scala {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Swift {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Swift {
    fn default() -> Self {
        Self {
            start: "//start//".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Typescript {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Typescript {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Racket {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Racket {
    fn default() -> Self {
        Self {
            start: ";;start;".to_owned(),
            end: ";;end;".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Erlang {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Erlang {
    fn default() -> Self {
        Self {
            start: "%%start%".to_owned(),
            end: "%%end%".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Elixir {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Elixir {
    fn default() -> Self {
        Self {
            start: "##start#".to_owned(),
            end: "##end#".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dart {
    pub start: String,
    pub end: String,
    pub inject_start: String,
    pub inject_end: String,
}
impl Default for Dart {
    fn default() -> Self {
        Self {
            start: "//start/".to_owned(),
            end: "//end/".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
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
    pub question_url: String,
    pub graphql: String,
    pub all_problem_api: String,
    pub submit: String,
    pub test: String,
    pub submissions: String,
    pub favorites: String,
}

impl Urls {
    pub fn new(suffix: &str) -> Self {
        let suffix = match suffix {
            "cn" => "cn",
            "com" => "com",
            _ => "com",
        };
        Self {
            origin: format!("https://leetcode.{}", suffix),
            graphql: format!("https://leetcode.{}/graphql", suffix),
            question_url: format!("https://leetcode.{}/problems/$slug/", suffix),
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

impl Default for Urls {
    fn default() -> Self {
        let suffix = "com";
        Self {
            origin: format!("https://leetcode.{}", suffix),
            graphql: format!("https://leetcode.{}/graphql", suffix),
            question_url: format!("https://leetcode.{}/problems/$slug/", suffix),
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
