use std::fmt::Display;

use serde::{Deserialize, Serialize};

const START_SLASH: &str = "// start /";
const END_SLASH: &str = "// end /";
const START_HASH: &str = "## start #";
const END_HASH: &str = "## end #";
const START_DASH: &str = "-- start -";
const END_DASH: &str = "-- end -";

macro_rules! skipfmt {
    ($($code:tt)*) => { $($code)* }
}

macro_rules! lang_macro {
    ($($struct_name:ident), *) => {
        paste::paste! {
            #[derive(Clone)]
            #[derive(Debug)]
            #[derive(Default)]
            #[derive(PartialEq, Eq)]
            #[derive(Serialize, Deserialize)]
            pub struct SupportLang {
                $(
                    #[serde(default)]
                    pub [<$struct_name:lower>]: $struct_name,
                )*
            }
        }
        $(
            #[derive(Clone)]
            #[derive(Debug)]
            #[derive(PartialEq, Eq)]
            #[derive(Serialize, Deserialize)]
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
macro_rules! defaults {
    ($lang:ident, $start:expr, $end:expr, $inject_start:expr, $inject_end:expr) => {
        impl Default for $lang {
            fn default() -> Self {
                Self {
                    start: $start.to_owned(),
                    end: $end.to_owned(),
                    inject_start: $inject_start.to_owned(),
                    inject_end: $inject_end.to_owned(),
                }
            }
        }
    };
    ($lang:ident, $start:expr, $end:expr, $inject_start:expr) => {
        impl Default for $lang {
            fn default() -> Self {
                Self {
                    start: $start.to_owned(),
                    end: $end.to_owned(),
                    inject_start: $inject_start.to_owned(),
                    inject_end: String::new(),
                }
            }
        }
    };
    ($lang:ident, $start:expr, $end:expr, $inject_end:expr) => {
        impl Default for $lang {
            fn default() -> Self {
                Self {
                    start: $start.to_owned(),
                    end: $end.to_owned(),
                    inject_start: String::new(),
                    inject_end: $inject_end.to_owned(),
                }
            }
        }
    };
    ($lang:ident, $start:expr, $end:expr) => {
        impl Default for $lang {
            fn default() -> Self {
                Self {
                    start: $start.to_owned(),
                    end: $end.to_owned(),
                    inject_start: String::new(),
                    inject_end: String::new(),
                }
            }
        }
    };
}

skipfmt!(
defaults!(C         , START_SLASH , END_SLASH);
defaults!(Cpp       , START_SLASH , END_SLASH);
defaults!(Csharp    , START_SLASH , END_SLASH);
defaults!(Dart      , START_SLASH , END_SLASH);
defaults!(Golang    , START_SLASH , END_SLASH);
defaults!(Java      , START_SLASH , END_SLASH);
defaults!(Javascript, START_SLASH , END_SLASH);
defaults!(Kotlin    , START_SLASH , END_SLASH);
defaults!(Php       , START_SLASH , END_SLASH);
defaults!(React     , START_SLASH , END_SLASH);
defaults!(Scala     , START_SLASH , END_SLASH);
defaults!(Swift     , START_SLASH , END_SLASH);
defaults!(Typescript, START_SLASH , END_SLASH);

defaults!(Bash      , START_HASH  , END_HASH );
defaults!(Elixir    , START_HASH  , END_HASH );
defaults!(Python    , START_HASH  , END_HASH );
defaults!(Python3   , START_HASH  , END_HASH );
defaults!(Ruby      , START_HASH  , END_HASH );

defaults!(Mssql     , START_DASH  , END_DASH );
defaults!(Mysql     , START_DASH  , END_DASH );
defaults!(Oraclesql , START_DASH  , END_DASH );
defaults!(Postgresql, START_DASH  , END_DASH );

defaults!(Racket    , ";; start ;", ";; end ;");
defaults!(Erlang    , "%% start %", "%% end %");
);

defaults!(
    Rust,
    START_SLASH,
    END_SLASH,
    "struct Solution;\n",
    "
fn main() {
    let res = Solution::function(todo!());
    dbg!(&res);
    assert_eq!(res, todo!());
}"
);

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Urls {
    pub origin: String,
    pub question_url: String,
    pub graphql: String,
    pub all_problem_api: String,
    pub submit: String,
    pub test: String,
    pub submissions: String,
    pub favorites: String,
    pub points: String,
}
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
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
    pub fn new(suffix: Suffix) -> Self {
        let suffix = match suffix {
            Suffix::Cn => "cn",
            Suffix::Com => "com",
        };
        Self {
            origin: format!("https://leetcode.{}", suffix),
            graphql: format!("https://leetcode.{}/graphql", suffix),
            question_url: format!("https://leetcode.{}/problems/$slug/", suffix),
            all_problem_api: format!("https://leetcode.{}/api/problems/$category", suffix),
            submit: format!("https://leetcode.{}/problems/$slug/submit/", suffix),
            test: format!(
                "https://leetcode.{}/problems/$slug/interpret_solution/",
                suffix
            ),
            submissions: format!("https://leetcode.{}/submissions/detail/$id/check/", suffix),
            favorites: format!("https://leetcode.{}/list/api/questions", suffix),
            points: format!("https://leetcode.{}/points/api/total/", suffix),
        }
    }

    pub fn mod_all_pb_api(&self, category: &str) -> String {
        self.all_problem_api
            .replace("$category", category)
    }

    pub fn mod_submit(&self, slug: &str) -> String {
        self.submit.replace("$slug", slug)
    }

    pub fn mod_test(&self, slug: &str) -> String {
        self.test.replace("$slug", slug)
    }

    pub fn mod_submissions(&self, id: &str) -> String {
        self.submissions.replace("$id", id)
    }
    pub fn get_qs_url(&self, slug: &str) -> String {
        self.question_url.replace("$slug", slug)
    }
}

impl Default for Urls {
    fn default() -> Self {
        Self::new(Suffix::default())
    }
}
