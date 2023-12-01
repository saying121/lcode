use std::fmt::Display;

use miette::{IntoDiagnostic, Result};
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
};
use sea_orm::sea_query::{self, OnConflict};
use serde::{Deserialize, Serialize};

use crate::{
    config::global::glob_user_config,
    dao::InsertToDB,
    entities::detail,
    render::{pre_render, Render},
};

use self::question::*;

mod my_metadata_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    use super::question::MetaData;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<MetaData, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(serde_json::from_str(&s).unwrap_or_default())
    }
    pub fn serialize<S>(v: &MetaData, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let a = serde_json::to_string(v).unwrap_or_default();

        serializer.serialize_str(&a)
    }
}
mod my_stats_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    use super::question::Stats;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Stats, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(serde_json::from_str(&s).unwrap_or_default())
    }
    pub fn serialize<S>(v: &Stats, deserializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = serde_json::to_string(v).unwrap_or_default();

        deserializer.serialize_str(&s)
    }
}

/// a question's detail
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Question {
    #[serde(default)]
    pub qs_slug: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default, with = "my_stats_serde")]
    pub stats: Stats,
    #[serde(default, alias = "sampleTestCase")]
    pub sample_test_case: String,
    #[serde(default, alias = "exampleTestcases")]
    pub example_testcases: String,
    #[serde(default, alias = "metaData", with = "my_metadata_serde")]
    pub meta_data: MetaData,
    #[serde(default, alias = "translatedTitle")]
    pub translated_title: Option<String>,
    #[serde(default, alias = "translatedContent")]
    pub translated_content: Option<String>,
    #[serde(default)]
    pub hints: Vec<String>,
    #[serde(default, alias = "mysqlSchemas")]
    pub mysql_schemas: Vec<String>,
    #[serde(default, alias = "dataSchemas")]
    pub data_schemas: Vec<String>,
    #[serde(default, alias = "questionId")]
    pub question_id: String,
    #[serde(default, alias = "questionTitle")]
    pub question_title: Option<String>,
    #[serde(default, alias = "isPaidOnly")]
    pub is_paid_only: bool,
    #[serde(default, alias = "codeSnippets")]
    pub code_snippets: Option<Vec<CodeSnippet>>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub difficulty: String,
    #[serde(alias = "topicTags")]
    pub topic_tags: Vec<TopicTags>,
}

impl Question {
    pub fn from_serde(v: serde_json::Value, title_slug: String) -> Result<Self> {
        let mut v: Self = serde_json::from_value(v).into_diagnostic()?;

        v.qs_slug = Some(title_slug);

        Ok(v)
    }
}

// impl Question {
#[async_trait::async_trait]
impl InsertToDB for Question {
    type Value = u32;
    type Entity = detail::Entity;
    type Model = detail::Model;
    type ActiveModel = detail::ActiveModel;

    fn to_model(&self, question_id: Self::Value) -> Self::Model {
        Self::Model {
            id: question_id,
            content: serde_json::to_string(self).unwrap_or_default(),
        }
    }
    fn on_conflict() -> OnConflict {
        sea_query::OnConflict::column(detail::Column::Id)
            .update_columns([detail::Column::Id, detail::Column::Content])
            .to_owned()
    }
}

impl Render for Question {
    fn to_md_str(&self) -> String {
        let mut md_str = pre_render(self);
        if !self.mysql_schemas.is_empty() {
            let str = format!(
                "\n\
                ```sql\n\
                {}\n\
                ```\n\
                ",
                self.mysql_schemas.join("\n")
            );
            md_str.push_str(&str);
        }
        md_str
    }
    fn to_tui_mdvec(&self, width: usize) -> Vec<String> {
        use crate::render::to_sub_sup_script;
        let content = if glob_user_config().translate {
            self.translated_content
                .as_deref()
                .unwrap_or(
                    self.content
                        .as_deref()
                        .unwrap_or_default(),
                )
        } else {
            self.translated_content
                .as_deref()
                .unwrap_or_default()
        };

        let content = to_sub_sup_script(content);

        let a = html2text::from_read(content.as_bytes(), width);
        let res: Vec<String> = a
            .replace("\\\"", "\"")
            .replace("\\n", "")
            .replace("\n\n\n", "\n")
            .trim_matches(|c| c == '"' || c == '\n' || c == ' ')
            .split('\n')
            .map(|v| v.to_owned())
            .collect();

        let topic = self
            .topic_tags
            .iter()
            .map(|v| {
                if glob_user_config().translate {
                    if v.translated_name.is_none() {
                        v.name.clone()
                    } else {
                        v.translated_name
                            .clone()
                            .unwrap_or_default()
                    }
                } else {
                    v.name.clone()
                }
            })
            .collect::<Vec<String>>()
            .join(", ");

        // let t_case = format!("```\n{}\n```", self.example_testcases);
        let res1 = vec![
            format!(
            "* ID: {id:07} | Passing rate: {rt:.6} | PaidOnly: {pd:6} | Difficulty: {di}",
            id = self.question_id,
            rt = self
                .stats
                .ac_rate
                ,
            pd = self.is_paid_only,
            di = self.difficulty,
        ),
            format!("* Topic: {}", topic),
            String::new(),
        ];

        [res1, res].concat()
    }

    fn to_tui_vec(&self) -> Vec<Line> {
        use crate::render::to_sub_sup_script;
        use scraper::Html;
        let user = glob_user_config();

        let content = if user.translate {
            self.translated_content
                .as_deref()
                .unwrap_or(
                    self.content
                        .as_deref()
                        .unwrap_or_default(),
                )
        } else {
            self.content
                .as_deref()
                .unwrap_or_default()
        };

        let content = to_sub_sup_script(content);

        let frag = Html::parse_fragment(&content);
        let res = frag
            .root_element()
            .text()
            .fold(String::new(), |acc, e| acc + e);

        let res: Vec<Line> = res
            .replace("\\\"", "\"")
            .replace("\\\\", "")
            .replace("\\n", "\n")
            .replace("\\t", "    ")
            .replace("\n\n\n", "\n")
            .trim_matches(|c| c == '"' || c == '\n' || c == ' ')
            .split('\n')
            .map(|v| Line::from(v.to_owned()))
            .collect();

        let topic = self
            .topic_tags
            .iter()
            .map(|v| {
                if user.translate {
                    if v.translated_name.is_none() {
                        v.name.clone()
                    } else {
                        v.translated_name
                            .clone()
                            .unwrap_or_default()
                    }
                } else {
                    v.name.clone()
                }
            })
            .collect::<Vec<String>>()
            .join(", ");

        let res1 = vec![
            Line::from(vec![
                Span::styled("• ID: ", Style::default()),
                Span::styled(self.question_id.clone(), Style::default().bold()),
                Span::styled(" | Passing rate: ", Style::default()),
                Span::styled(self.stats.ac_rate.clone(), Style::default().bold()),
                Span::styled(" | PaidOnly: ", Style::default()),
                Span::styled(self.is_paid_only.to_string(), Style::default().bold()),
                Span::styled(" | Difficulty: ", Style::default()),
                Span::styled(self.difficulty.clone(), Style::default().bold()),
            ]),
            Line::from(vec![
                Span::styled("• Topic: ", Style::default().bold()),
                Span::styled(topic, Style::default()),
            ]),
            Line::from(vec![
                Span::styled("• Url: ", Style::default()),
                Span::styled(
                    user.get_qsurl(
                        self.qs_slug
                            .as_deref()
                            .unwrap_or_default(),
                    ),
                    Style::default().bold(),
                ),
            ]),
            Line::from(String::new()),
        ];

        [res1, res].concat()
    }
    fn to_rendered_str(&self, col: u16, row: u16) -> Result<String> {
        use pulldown_cmark_mdcat::{Settings, TerminalProgram, TerminalSize, Theme};
        use syntect::parsing::SyntaxSet;

        use crate::render::{rendering, StTy};

        let md_str = pre_render(self);

        let term_size = TerminalSize {
            columns: col,
            rows: row,
            ..Default::default()
        };

        let set = Settings {
            terminal_capabilities: TerminalProgram::detect().capabilities(),
            terminal_size: term_size,
            syntax_set: &SyntaxSet::load_defaults_newlines(),
            theme: Theme::default(),
        };

        let res = rendering(&set, &md_str, StTy::Str)?;

        Ok(res)
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let user = glob_user_config();
        let title = if user.translate {
            self.translated_title
                .as_ref()
                .map_or(self.title.clone(), |v| v.clone())
                .as_str()
                .trim_matches('"')
                .to_owned()
        } else {
            self.title.clone()
        };

        let topic = self
            .topic_tags
            .iter()
            .map(|v| {
                if user.translate {
                    if v.translated_name.is_none() {
                        v.name.clone()
                    } else {
                        v.translated_name
                            .clone()
                            .unwrap_or_default()
                    }
                } else {
                    v.name.clone()
                }
            })
            .collect::<Vec<String>>()
            .join(", ");

        let t_case = format!("```\n{}\n```", self.example_testcases);
        format!(
            "# {tit:62}\n\
            \n\
        * ID: {id:07} | Passing rate: {rt:.6} | PaidOnly: {pd:6} | Difficulty: {di}\n\
            * Url: {url}\n\
            * Topic: {tp}\n\
            \n\
            ## Test Case:\n\
            \n\
            {t_case}\n",
            tit = title,
            id = self.question_id,
            rt = self.stats.ac_rate,
            pd = self.is_paid_only,
            di = self.difficulty,
            tp = topic,
            t_case = t_case,
            url = user.get_qsurl(
                self.qs_slug
                    .as_deref()
                    .unwrap_or_default()
            )
        )
        .fmt(f)
    }
}

pub mod question {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
    pub struct Stats {
        #[serde(alias = "totalAccepted")]
        pub total_accepted: String,
        #[serde(alias = "totalSubmission")]
        pub total_submission: String,
        #[serde(alias = "totalAcceptedRaw")]
        pub total_accepted_raw: usize,
        #[serde(alias = "totalSubmissionRaw")]
        pub total_submission_raw: usize,
        #[serde(alias = "acRate")]
        pub ac_rate: String,
    }
    /// metadata
    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
    pub struct MetaData {
        #[serde(default)]
        pub name: String,
        #[serde(default)]
        pub params: Vec<Param>,
        #[serde(default)]
        pub r#return: Return,
    }

    /// nest field
    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
    pub struct Param {
        #[serde(default)]
        pub name: String,
        #[serde(default)]
        pub r#type: String,
        // pub dealloc: bool,
    }

    /// nest field
    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
    pub struct Return {
        #[serde(default)]
        pub r#type: String,
        // pub dealloc: bool,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
    /// language and it's snippet
    pub struct CodeSnippet {
        #[serde(default)]
        pub lang: String,
        #[serde(default, alias = "langSlug")]
        pub lang_slug: String,
        #[serde(default)]
        pub code: String,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
    pub struct TopicTags {
        #[serde(default)]
        pub name: String,
        #[serde(default)]
        pub slug: String,
        #[serde(default, alias = "translatedName")]
        pub translated_name: Option<String>,
    }
}
