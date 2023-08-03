use serde::{Deserialize, Serialize};

use self::question::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// a question's detail
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

use serde_json::Value;
use tracing::{debug, instrument};
impl Question {
    /// parser json to detail question,if field not exists will use default
    ///
    /// * `v`: serde_json::Value
    #[instrument]
    pub fn parser_question(v: Value) -> Question {
        let def_v = Value::default();

        let temp = "content";
        debug!("Deserialize {}", temp);
        let content = match v.get(temp) {
            Some(it) => Some(it.to_string()),
            None => None,
        };

        let temp = "questionTitle";
        debug!("Deserialize {}", temp);
        let question_title = match v.get(temp) {
            Some(it) => Some(it.to_string()),
            None => None,
        };

        let temp = "translatedTitle";
        debug!("Deserialize {}", temp);
        let translated_title = match v.get(temp) {
            Some(it) => Some(it.to_string()),
            None => None,
        };

        let temp = "translatedContent";
        debug!("Deserialize {}", temp);
        let translated_content = match v.get(temp) {
            Some(it) => Some(it.to_string()),
            None => None,
        };

        let temp = "stats";
        debug!("Deserialize {}", temp);
        let stats = serde_json::from_str(
            v.get(temp)
                .and_then(|v| v.as_str())
                .unwrap_or_default(),
        )
        .unwrap_or_default();

        let temp = "sampleTestCase";
        debug!("Deserialize {}", temp);
        let sample_test_case = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let temp = "exampleTestcases";
        debug!("Deserialize {}", temp);
        let example_testcases = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let temp = "metaData";
        debug!("Deserialize {}", temp);
        let meta_data = serde_json::from_str(
            v.get(temp)
                .and_then(|v| v.as_str())
                .unwrap_or_default(),
        )
        .unwrap_or_default();

        let temp = "hints";
        debug!("Deserialize {}", temp);
        let hints = serde_json::from_value(
            v.get(temp)
                .unwrap_or(&def_v)
                .clone(),
        )
        .unwrap_or_default();

        let temp = "mysqlSchemas";
        debug!("Deserialize {}", temp);
        let mysql_schemas = serde_json::from_value(
            v.get(temp)
                .unwrap_or(&def_v)
                .clone(),
        )
        .unwrap_or_default();

        let temp = "dataSchemas";
        debug!("Deserialize {}", temp);
        let data_schemas = serde_json::from_value(
            v.get(temp)
                .unwrap_or(&def_v)
                .clone(),
        )
        .unwrap_or_default();

        let temp = "questionId";
        debug!("Deserialize {}", temp);
        let question_id = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let temp = "isPaidOnly";
        debug!("Deserialize {}", temp);
        let is_paid_only = v
            .get(temp)
            .and_then(|v| v.as_bool())
            .unwrap_or_default();

        let temp = "codeSnippets";
        debug!("Deserialize {}", temp);
        let code_snippets = serde_json::from_value(
            v.get(temp)
                .unwrap_or(&def_v)
                .clone(),
        )
        .unwrap_or_default();

        let temp = "title";
        debug!("Deserialize {}", temp);
        let title = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let temp = "difficulty";
        debug!("Deserialize {}", temp);
        let difficulty = v
            .get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let temp = "topicTags";
        debug!("Deserialize {}", temp);
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
        total_accepted: String,
        #[serde(alias = "totalSubmission")]
        total_submission: String,
        #[serde(alias = "totalAcceptedRaw")]
        total_accepted_raw: usize,
        #[serde(alias = "totalSubmissionRaw")]
        total_submission_raw: usize,
        #[serde(alias = "acRate")]
        ac_rate: String,
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
        name: String,
        slug: String,
        #[serde(alias = "translatedName")]
        translated_name: String,
    }
}
