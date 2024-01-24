use sea_orm::sea_query::{self, OnConflict};
use serde::{Deserialize, Serialize};

use self::question::*;
use crate::{dao::InsertToDB, entities::detail};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct QuestionData {
    #[serde(default)]
    pub data: Detail,
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Detail {
    #[serde(default)]
    pub question: Question,
}

/// this field all from
/// `String`(json from leetcode website) ->
/// `Struct`(`Question` field) ->
/// `String`(database store for correct deserialize to `Question` field)
macro_rules! my_serde {
    ($($struct_name:ident),*) => {
        paste::paste! {
            $(
                mod [<$struct_name:snake:lower _serde>] {
                    use serde::{Deserialize, Deserializer, Serializer};

                    use super::question::$struct_name;

                    pub fn deserialize<'de, D>(deserializer: D) -> Result<$struct_name, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        let s = String::deserialize(deserializer)?;

                        Ok(serde_json::from_str(&s).unwrap_or_default())
                    }

                    pub fn serialize<S>(v: &$struct_name, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                    {
                        let a = serde_json::to_string(v).unwrap_or_default();

                        serializer.serialize_str(&a)
                    }
                }
            )*
        }
    };
}
my_serde!(MetaData, Stats, EnvInfo);

/// a question's detail
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Question {
    #[serde(default)]
    pub qs_slug:            Option<String>,
    #[serde(default)]
    pub content:            Option<String>,
    #[serde(default, with = "stats_serde")]
    pub stats:              Stats,
    #[serde(default, alias = "sampleTestCase")]
    pub sample_test_case:   String,
    #[serde(default, alias = "exampleTestcases")]
    pub example_testcases:  String,
    #[serde(default, alias = "metaData", with = "meta_data_serde")]
    pub meta_data:          MetaData,
    #[serde(default, alias = "translatedTitle")]
    pub translated_title:   Option<String>,
    #[serde(default, alias = "translatedContent")]
    pub translated_content: Option<String>,
    #[serde(default)]
    pub hints:              Vec<String>,
    #[serde(default, alias = "mysqlSchemas")]
    pub mysql_schemas:      Vec<String>,
    #[serde(default, alias = "dataSchemas")]
    pub data_schemas:       Vec<String>,
    #[serde(default, alias = "questionId")]
    pub question_id:        String,
    #[serde(default, alias = "questionTitle")]
    pub question_title:     Option<String>,
    #[serde(default, alias = "isPaidOnly")]
    pub is_paid_only:       bool,
    #[serde(default, alias = "codeSnippets")]
    pub code_snippets:      Option<Vec<CodeSnippet>>,
    #[serde(default)]
    pub title:              String,
    #[serde(default)]
    pub difficulty:         String,
    #[serde(alias = "topicTags")]
    pub topic_tags:         Vec<TopicTags>,
    #[serde(alias = "enableRunCode")]
    pub enable_run_code:    bool,
    #[serde(default, alias = "envInfo", with = "env_info_serde")]
    pub env_info:           EnvInfo,
}

impl InsertToDB for Question {
    type Value = u32;
    type Entity = detail::Entity;
    type Model = detail::Model;
    type ActiveModel = detail::ActiveModel;

    fn to_model(&self, question_id: Self::Value) -> Self::Model {
        Self::Model {
            id:      question_id,
            content: serde_json::to_string(self).unwrap_or_default(),
        }
    }
    fn on_conflict() -> OnConflict {
        sea_query::OnConflict::column(detail::Column::Id)
            .update_columns([detail::Column::Id, detail::Column::Content])
            .to_owned()
    }
}

pub mod question {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};

    macro_rules! env_info_macro {
        ($($lang_name:ident),*) => {
            #[derive(Clone)]
            #[derive(Debug)]
            #[derive(Default)]
            #[derive(PartialEq, Eq)]
            #[derive(Serialize, Deserialize)]
            pub struct EnvInfo {
                $(
                    #[serde(default)]
                    $lang_name : Vec<String>,
                )*
            }
            impl Display for EnvInfo {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let mut res = String::new();
                    $(
                        if !self.$lang_name.is_empty() {
                            let pat = format!("{}\n", self.$lang_name.join(":\n"));
                            let pat = format!("\n## {}", pat);
                            res.push_str(&pat);
                        }
                    )*
                    let res = html2text::from_read(res.as_bytes(), 80);
                    res.fmt(f)
                }
            }
        };
    }
    env_info_macro!(
        bash, c, cpp, csharp, dart, elixir, erlang, golang, java, javascript, kotlin, mssql, mysql,
        oraclesql, postgresql, php, python, python3, pythondata, pythonml, racket, react, ruby,
        rust, scala, swift, typescript
    );

    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    #[derive(Serialize, Deserialize)]
    pub struct Stats {
        #[serde(alias = "totalAccepted")]
        pub total_accepted:       String,
        #[serde(alias = "totalSubmission")]
        pub total_submission:     String,
        #[serde(alias = "totalAcceptedRaw")]
        pub total_accepted_raw:   usize,
        #[serde(alias = "totalSubmissionRaw")]
        pub total_submission_raw: usize,
        #[serde(alias = "acRate")]
        pub ac_rate:              String,
    }
    /// metadata
    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    #[derive(Serialize, Deserialize)]
    pub struct MetaData {
        #[serde(default)]
        pub name:     String,
        #[serde(default)]
        pub params:   Vec<Param>,
        #[serde(default)]
        pub r#return: Return,
    }

    /// nest field
    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    #[derive(Serialize, Deserialize)]
    pub struct Param {
        #[serde(default)]
        pub name:   String,
        #[serde(default)]
        pub r#type: String,
        // pub dealloc: bool,
    }

    /// nest field
    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    #[derive(Serialize, Deserialize)]
    pub struct Return {
        #[serde(default)]
        pub r#type: String,
        // pub dealloc: bool,
    }

    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    #[derive(Serialize, Deserialize)]
    /// language and it's snippet
    pub struct CodeSnippet {
        #[serde(default)]
        pub lang:      String,
        #[serde(default, alias = "langSlug")]
        pub lang_slug: String,
        #[serde(default)]
        pub code:      String,
    }

    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    #[derive(Serialize, Deserialize)]
    pub struct TopicTags {
        #[serde(default)]
        pub name:            String,
        #[serde(default)]
        pub slug:            String,
        #[serde(default, alias = "translatedName")]
        pub translated_name: Option<String>,
    }
}
