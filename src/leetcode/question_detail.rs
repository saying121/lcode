use std::fmt::Display;

use colored::Colorize;
use miette::Result;
use serde::{Deserialize, Serialize};

use crate::{config::global::global_user_config, render::Render};

use self::question::*;

/// a question's detail
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Question {
    pub content: Option<String>,
    pub stats: Stats,
    #[serde(alias = "sampleTestCase")]
    pub sample_test_case: String,
    #[serde(alias = "exampleTestcases")]
    pub example_testcases: String,
    #[serde(alias = "metaData")]
    pub meta_data: MetaData,
    #[serde(alias = "translatedTitle")]
    pub translated_title: Option<String>,
    #[serde(alias = "translatedContent")]
    pub translated_content: Option<String>,
    pub hints: Vec<String>,
    #[serde(alias = "mysqlSchemas")]
    pub mysql_schemas: Vec<String>,
    #[serde(alias = "dataSchemas")]
    pub data_schemas: Vec<String>,
    #[serde(alias = "questionId")]
    pub question_id: String,
    #[serde(alias = "questionTitle")]
    pub question_title: Option<String>,
    #[serde(alias = "isPaidOnly")]
    pub is_paid_only: bool,
    #[serde(alias = "codeSnippets")]
    pub code_snippets: Vec<CodeSnippet>,
    pub title: String,
    pub difficulty: String,
    #[serde(alias = "topicTags")]
    pub topic_tags: Vec<TopicTags>,
}

impl Render for Question {
    fn to_tui_mdvec(&self, width: usize) -> Vec<String> {
        use crate::render::gen_sub_sup_script;
        let user = global_user_config();

        let content = match user.translate {
            true => self
                .translated_content
                .as_ref()
                .unwrap_or(
                    self.content
                        .as_ref()
                        .unwrap_or(&"".to_string()),
                )
                .to_owned(),
            false => self
                .translated_content
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_owned(),
        };

        let content = gen_sub_sup_script(content);

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
    fn to_rendered_str(&self, col: u16, row: u16) -> Result<String> {
        use pulldown_cmark_mdcat::{Settings, TerminalProgram, TerminalSize, Theme};
        use syntect::parsing::SyntaxSet;

        use crate::render::{pre_render, rendering, StTy};

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
        let user = global_user_config();
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
            "# {tit:62} \n\
            * ID: {id:07} | Passing rate: {rt:.6} | PaidOnly: {pd:6} | Difficulty: {di} \n\
            * Topic: \n{tp} \n\
            ## Test Case: \n{t_case}",
            tit = title,
            id = self.question_id,
            rt = self
                .stats
                .ac_rate
                .yellow()
                .italic(),
            pd = self.is_paid_only,
            di = self.difficulty.bold(),
            tp = topic,
            t_case = t_case,
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
    pub fn parser_question(v: Value) -> Question {
        let def_v = Value::default();

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
            .to_string();

        let temp = "exampleTestcases";
        trace!("Deserialize {}", temp);
        let example_testcases = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

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
                .unwrap_or(&def_v)
                .clone(),
        )
        .unwrap_or_default();

        let temp = "mysqlSchemas";
        trace!("Deserialize {}", temp);
        let mysql_schemas = serde_json::from_value(
            v.get(temp)
                .unwrap_or(&def_v)
                .clone(),
        )
        .unwrap_or_default();

        let temp = "dataSchemas";
        trace!("Deserialize {}", temp);
        let data_schemas = serde_json::from_value(
            v.get(temp)
                .unwrap_or(&def_v)
                .clone(),
        )
        .unwrap_or_default();

        let temp = "questionId";
        trace!("Deserialize {}", temp);
        let question_id = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

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
                .unwrap_or(&def_v)
                .clone(),
        )
        .unwrap_or_default();

        let temp = "title";
        trace!("Deserialize {}", temp);
        let title = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let temp = "difficulty";
        trace!("Deserialize {}", temp);
        let difficulty = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let temp = "topicTags";
        trace!("Deserialize {}", temp);
        let topic_tags = serde_json::from_value(
            v.get(temp)
                .unwrap_or(&def_v)
                .clone(),
        )
        .unwrap_or_default();

        Question {
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
