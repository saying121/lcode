use std::fmt::Display;

use miette::Result;
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
};
use serde::{Deserialize, Serialize};

use crate::{
    config::global::glob_user_config,
    render::{pre_render, Render},
};

use self::question::*;

// fn my_metadata_deserialize<'de, D>(deserializer: D) -> Result<MetaData, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//
//     let v = serde_json::from_str(&s).unwrap_or_default();
//     Ok(v)
// }
// fn my_stats_deserialize<'de, D>(deserializer: D) -> Result<Stats, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//
//     let v = serde_json::from_str(&s).unwrap_or_default();
//     Ok(v)
// }

/// a question's detail
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Question {
    #[serde(default)]
    pub qs_slug: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    // #[serde(default, deserialize_with = "my_stats_deserialize")]
    #[serde(default)]
    pub stats: Stats,
    #[serde(default, alias = "sampleTestCase")]
    pub sample_test_case: String,
    #[serde(default, alias = "exampleTestcases")]
    pub example_testcases: String,
    // #[serde(
    //     default,
    //     alias = "metaData",
    //     deserialize_with = "my_metadata_deserialize"
    // )]
    #[serde(default, alias = "metaData")]
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
    pub code_snippets: Vec<CodeSnippet>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub difficulty: String,
    #[serde(alias = "topicTags")]
    pub topic_tags: Vec<TopicTags>,
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
        let content = match glob_user_config().translate {
            true => self
                .translated_content
                .as_deref()
                .unwrap_or(
                    self.content
                        .as_deref()
                        .unwrap_or_default(),
                ),
            false => self
                .translated_content
                .as_deref()
                .unwrap_or_default(),
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
                let st = match glob_user_config().translate {
                    true => &v.translated_name,
                    false => &v.name,
                };
                st.to_string()
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

        let content = match user.translate {
            true => self
                .translated_content
                .as_deref()
                .unwrap_or(
                    self.content
                        .as_deref()
                        .unwrap_or_default(),
                ),
            false => self
                .content
                .as_deref()
                .unwrap_or_default(),
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
                let st = match user.translate {
                    true => &v.translated_name,
                    false => &v.name,
                };
                st.to_string()
            })
            .collect::<Vec<String>>()
            .join(", ");

        let res1 = vec![
            Line::from(vec![
                Span::styled("• ID: ", Style::default()),
                Span::styled(self.question_id.clone(), Style::default().bold()),
                Span::styled(" | Passing rate: ", Style::default()),
                Span::styled(self.stats.ac_rate.to_owned(), Style::default().bold()),
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

        let res = rendering(set, md_str, StTy::Str)?;

        Ok(res)
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let user = glob_user_config();
        let title = match user.translate {
            true => self
                .translated_title
                .as_ref()
                .map_or(self.title.to_owned(), |v| v.clone())
                .as_str()
                .trim_matches('"')
                .to_owned(),
            false => self.title.to_owned(),
        };

        let topic = self
            .topic_tags
            .iter()
            .map(|v| {
                let st = match user.translate {
                    true => &v.translated_name,
                    false => &v.name,
                };
                st.to_string()
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

use serde_json::Value;
use tracing::{instrument, trace};

impl Question {
    /// parser json to detail question,if field not exists will use default
    ///
    /// * `v`: serde_json::Value
    #[instrument(skip(v))]
    pub fn parser_question(v: Value, slug: String) -> Question {
        let temp = "content";
        trace!("Deserialize {}", temp);
        let content = v
            .get(temp)
            .map(|it| it.to_string());

        let temp = "questionTitle";
        trace!("Deserialize {}", temp);
        let question_title = v
            .get(temp)
            .map(|it| it.to_string());

        let temp = "translatedTitle";
        trace!("Deserialize {}", temp);
        let translated_title = v
            .get(temp)
            .map(|it| it.to_string());

        let temp = "translatedContent";
        trace!("Deserialize {}", temp);
        let translated_content = v
            .get(temp)
            .map(|it| it.to_string());

        let temp = "stats";
        trace!("Deserialize {}", temp);
        let stats = serde_json::from_str(
            v.get(temp)
                .and_then(|v| v.as_str())
                .unwrap_or_default(),
        )
        .unwrap_or_default();

        let temp = "sampleTestCase";
        trace!("Deserialize {}", temp);
        let sample_test_case = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_owned();

        let temp = "exampleTestcases";
        trace!("Deserialize {}", temp);
        let example_testcases = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_owned();

        let temp = "metaData";
        trace!("Deserialize {}", temp);
        let meta_data = serde_json::from_str(
            v.get(temp)
                .and_then(|v| v.as_str())
                .unwrap_or_default(),
        )
        .unwrap_or_default();

        let temp = "hints";
        trace!("Deserialize {}", temp);
        let hints = serde_json::from_value(
            v.get(temp)
                .cloned()
                .unwrap_or_default(),
        )
        .unwrap_or_default();

        let temp = "mysqlSchemas";
        trace!("Deserialize {}", temp);
        let mysql_schemas = serde_json::from_value(
            v.get(temp)
                .cloned()
                .unwrap_or_default(),
        )
        .unwrap_or_default();

        let temp = "dataSchemas";
        trace!("Deserialize {}", temp);
        let data_schemas = serde_json::from_value(
            v.get(temp)
                .cloned()
                .unwrap_or_default(),
        )
        .unwrap_or_default();

        let temp = "questionId";
        trace!("Deserialize {}", temp);
        let question_id = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_owned();

        let temp = "isPaidOnly";
        trace!("Deserialize {}", temp);
        let is_paid_only = v
            .get(temp)
            .and_then(|v| v.as_bool())
            .unwrap_or_default();

        let temp = "codeSnippets";
        trace!("Deserialize {}", temp);
        let code_snippets = serde_json::from_value(
            v.get(temp)
                .cloned()
                .unwrap_or_default(),
        )
        .unwrap_or_default();

        let temp = "title";
        trace!("Deserialize {}", temp);
        let title = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_owned();

        let temp = "difficulty";
        trace!("Deserialize {}", temp);
        let difficulty = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_owned();

        let temp = "topicTags";
        trace!("Deserialize {}", temp);
        let topic_tags = serde_json::from_value(
            v.get(temp)
                .cloned()
                .unwrap_or_default(),
        )
        .unwrap_or_default();

        Question {
            qs_slug: Some(slug),
            content,
            stats,
            sample_test_case,
            example_testcases,
            meta_data,
            translated_title,
            translated_content,
            hints,
            mysql_schemas,
            data_schemas,
            question_id,
            question_title,
            is_paid_only,
            code_snippets,
            title,
            difficulty,
            topic_tags,
        }
    }
}

pub mod question {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct MetaData {
        pub name: String,
        pub params: Vec<Param>,
        pub r#return: Return,
    }

    /// nest field
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Param {
        pub name: String,
        pub r#type: String,
        // pub dealloc: bool,
    }

    /// nest field
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Return {
        pub r#type: String,
        // pub dealloc: bool,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    /// language and it's snippet
    pub struct CodeSnippet {
        pub lang: String,
        #[serde(alias = "langSlug")]
        pub lang_slug: String,
        pub code: String,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct TopicTags {
        pub name: String,
        pub slug: String,
        #[serde(alias = "translatedName")]
        pub translated_name: String,
    }
}
