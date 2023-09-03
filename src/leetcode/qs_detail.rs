use std::fmt::Display;

use miette::Result;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    config::global::glob_user_config,
    render::{pre_render, Render},
};

use self::question::*;

fn my_metadata_deserialize<'de, D>(deserializer: D) -> Result<MetaData, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let v = serde_json::from_str(&s).unwrap_or_default();
    Ok(v)
}
fn my_stats_deserialize<'de, D>(deserializer: D) -> Result<Stats, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let v = serde_json::from_str(&s).unwrap_or_default();
    Ok(v)
}

/// a question's detail
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Question {
    #[serde(default)]
    pub qs_slug: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default, deserialize_with = "my_stats_deserialize")]
    pub stats: Stats,
    #[serde(default, alias = "sampleTestCase")]
    pub sample_test_case: String,
    #[serde(default, alias = "exampleTestcases")]
    pub example_testcases: String,
    #[serde(
        default,
        alias = "metaData",
        deserialize_with = "my_metadata_deserialize"
    )]
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
        pre_render(self)
    }
    fn to_tui_mdvec(&self, width: usize) -> Vec<String> {
        use crate::render::gen_sub_sup_script;
        let user = glob_user_config();

        let content = match user.translate {
            true => self
                .translated_content
                .as_ref()
                .cloned()
                .unwrap_or(
                    self.content
                        .as_ref()
                        .cloned()
                        .unwrap_or_default(),
                ),
            false => self
                .translated_content
                .as_ref()
                .cloned()
                .unwrap_or_default(),
        };

        let content = gen_sub_sup_script(&content);

        let a = html2text::from_read(content.as_bytes(), width);
        let res: Vec<String> = a
            .replace("\\\"", "\"")
            .replace("\\n", "")
            .replace("\n\n\n", "\n")
            .trim_matches(|c| c == '"' || c == '\n' || c == ' ')
            .split('\n')
            .map(|v| v.to_string())
            .collect();

        let topic = self
            .topic_tags
            .iter()
            .map(|v| {
                let st = match user.translate {
                    true => &v.translated_name,
                    false => &v.name,
                };
                format!("{}", st)
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
            "".to_string(),
        ];

        [res1, res].concat()
    }

    fn to_tui_vec(&self) -> Vec<String> {
        use crate::render::gen_sub_sup_script;
        use scraper::Html;
        let user = glob_user_config();

        let content = match user.translate {
            true => self
                .translated_content
                .as_ref()
                .cloned()
                .unwrap_or(
                    self.content
                        .as_ref()
                        .cloned()
                        .unwrap_or_default(),
                ),
            false => self
                .content
                .as_ref()
                .cloned()
                .unwrap_or_default(),
        };

        let content = gen_sub_sup_script(&content);

        let frag = Html::parse_fragment(&content);
        let res = frag
            .root_element()
            .text()
            .fold(String::new(), |acc, e| acc + e);

        let res: Vec<String> = res
            .replace("\\\"", "\"")
            .replace("\\\\", "")
            .replace("\\n", "\n")
            .replace("\\t", "    ")
            .replace("\n\n\n", "\n")
            .trim_matches(|c| c == '"' || c == '\n' || c == ' ')
            .split('\n')
            .map(|v| v.to_string())
            .collect();

        let topic = self
            .topic_tags
            .iter()
            .map(|v| {
                let st = match user.translate {
                    true => &v.translated_name,
                    false => &v.name,
                };
                format!("{}", st)
            })
            .collect::<Vec<String>>()
            .join(", ");

        let res1 = vec![
            format!(
            "• ID: {id:07} | Passing rate: {rt:.6} | PaidOnly: {pd:6} | Difficulty: {di}",
            id = self.question_id,
            rt = self
                .stats
                .ac_rate
                ,
            pd = self.is_paid_only,
            di = self.difficulty,
        ),
            format!("• Topic: {}", topic),
            format!(
                "• Url: {}",
                user.get_qsurl(
                    self.qs_slug
                        .as_ref()
                        .unwrap_or(&"".to_string())
                )
            ),
            "".to_string(),
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

        let res = rendering(set, md_str, StTy::STR)?;

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
                format!("{}", st)
            })
            .collect::<Vec<String>>()
            .join(", ");

        let t_case = format!("```\n{}\n```", self.example_testcases);
        format!(
            "# {tit:62} \n\n\
        * ID: {id:07} | Passing rate: {rt:.6} | PaidOnly: {pd:6} | Difficulty: {di} \n\
            * Url: {url} \n\
            * Topic: {tp} \n\n\
            ## Test Case:\n\n{t_case}\n",
            tit = title,
            id = self.question_id,
            rt = self.stats.ac_rate,
            pd = self.is_paid_only,
            di = self.difficulty,
            tp = topic,
            t_case = t_case,
            url = user.get_qsurl(
                self.qs_slug
                    .as_ref()
                    .unwrap_or(&"".to_string())
            )
        )
        .fmt(f)
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
