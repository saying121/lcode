use std::fmt::Display;

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

// macro_rules! support_lang {
//     ($($struct_name:ident),*) => {
//         #[derive(Default, Clone, Debug, Serialize, Deserialize)]
//         pub struct SupportLang {
//             $(
//                 pub $struct_name: $struct_name,
//             )*
//         }
//     };
// }
// support_lang!(
//     Rust, Bash, C, Cpp, Csharp, Golang, Java, Javascript, Kotlin, Mysql, Php, Python3,
//     Python, Ruby, Scala, Swift, Typescript, Erlang, Elixir, Racket, Dart
// );

macro_rules! lang_macro {
    ($($struct_name:ident),*) => {
        $(
            #[derive(Clone, Debug, Serialize, Deserialize)]
            pub struct $struct_name {
                pub start: String,
                pub end: String,
                pub inject_start: String,
                pub inject_end: String,
            }
            impl $struct_name {
                /// `start`, `end`, `inject_end`, `inject_end`
                pub fn return_info(&self) -> (String, String, String, String) {
                    (
                        self.start.to_owned(),
                        self.end.to_owned(),
                        self.inject_start.to_owned(),
                        self.inject_end.to_owned(),
                    )
                }
            }
        )*
    };
}
lang_macro!(
    Rust, Bash, C, Cpp, Csharp, Golang, Java, Javascript, Kotlin, Mysql, Php, Python3,
    Python, Ruby, Scala, Swift, Typescript, Erlang, Elixir, Racket, Dart
);

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
impl Default for Bash {
    fn default() -> Self {
        Self {
            start: "##start#".to_owned(),
            end: "##end#".to_owned(),
            inject_start: String::new(),
            inject_end: String::new(),
        }
    }
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

impl Display for Cookies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("LEETCODE_SESSION={};csrftoken={};", self.session, self.csrf).fmt(f)
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
