use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// do repetitive work
macro_rules! lang_macro {
    ($($struct_name:ident), *) => {
        paste::paste! {
            #[derive(Default, Clone, Debug, Serialize, Deserialize)]
            pub struct SupportLang {
                $(
                    #[serde(default)]
                    pub [<$struct_name:lower>]: $struct_name,
                )*
            }
        }
        $(
            #[derive(Clone, Debug, Serialize, Deserialize)]
            pub struct $struct_name {
                pub start: String,
                pub end: String,
                pub inject_start: String,
                pub inject_end: String,
            }
            impl $struct_name {
                /// (`start`, `end`, `inject_start`, `inject_end`)
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
    Bash, C, Cpp, Csharp, Dart, Elixir, Erlang, Golang, Java, Javascript, Kotlin, Mssql, Mysql,
    Oraclesql, Postgresql, Php, Python, Python3, Racket, React, Ruby, Rust, Scala, Swift,
    Typescript
);

impl Default for Oraclesql {
    fn default() -> Self {
        Self {
            start:        "-- start -".to_owned(),
            end:          "-- end -".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for React {
    fn default() -> Self {
        Self {
            start:        "// start -".to_owned(),
            end:          "// end -".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Postgresql {
    fn default() -> Self {
        Self {
            start:        "-- start -".to_owned(),
            end:          "-- end -".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Mssql {
    fn default() -> Self {
        Self {
            start:        "-- start -".to_owned(),
            end:          "-- end -".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Rust {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: "struct Solution;\n".to_owned(),
            inject_end:   r#"
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
            start:        "## start #".to_owned(),
            end:          "## end #".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for C {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Cpp {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Csharp {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Golang {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Java {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Javascript {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Kotlin {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Mysql {
    fn default() -> Self {
        Self {
            start:        "-- start -".to_owned(),
            end:          "-- end -".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Php {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Python {
    fn default() -> Self {
        Self {
            start:        "## start #".to_owned(),
            end:          "## end #".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Python3 {
    fn default() -> Self {
        Self {
            start:        "## start #".to_owned(),
            end:          "## end #".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Ruby {
    fn default() -> Self {
        Self {
            start:        "## start #".to_owned(),
            end:          "## end #".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Scala {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Swift {
    fn default() -> Self {
        Self {
            start:        "// start //".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Typescript {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Racket {
    fn default() -> Self {
        Self {
            start:        ";; start ;".to_owned(),
            end:          ";; end ;".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Erlang {
    fn default() -> Self {
        Self {
            start:        "%% start %".to_owned(),
            end:          "%% end %".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Elixir {
    fn default() -> Self {
        Self {
            start:        "## start #".to_owned(),
            end:          "## end #".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}
impl Default for Dart {
    fn default() -> Self {
        Self {
            start:        "// start /".to_owned(),
            end:          "// end /".to_owned(),
            inject_start: String::new(),
            inject_end:   String::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Urls {
    pub origin:          String,
    pub question_url:    String,
    pub graphql:         String,
    pub all_problem_api: String,
    pub submit:          String,
    pub test:            String,
    pub submissions:     String,
    pub favorites:       String,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Suffix {
    Cn,
    #[default]
    Com,
}

impl Display for Suffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cn => "cn",
            Self::Com => "com",
        }
        .fmt(f)
    }
}

impl Urls {
    pub fn new(suffi: Suffix) -> Self {
        let suffix = match suffi {
            Suffix::Cn => "cn",
            Suffix::Com => "com",
        };
        Self {
            origin:          format!("https://leetcode.{}", suffix),
            graphql:         format!("https://leetcode.{}/graphql", suffix),
            question_url:    format!("https://leetcode.{}/problems/$slug/", suffix),
            all_problem_api: format!("https://leetcode.{}/api/problems/$category", suffix),
            submit:          format!("https://leetcode.{}/problems/$slug/submit/", suffix),
            test:            format!(
                "https://leetcode.{}/problems/$slug/interpret_solution/",
                suffix
            ),
            submissions:     format!("https://leetcode.{}/submissions/detail/$id/check/", suffix),
            favorites:       format!("https://leetcode.{}/list/api/questions", suffix),
        }
    }
}

impl Default for Urls {
    fn default() -> Self {
        Self::new(Suffix::default())
    }
}
