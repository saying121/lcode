use serde::{Deserialize, Serialize};

use self::question::*;

#[derive(Debug, Default, Serialize, Deserialize)]
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

pub mod question {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Serialize, Deserialize)]
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
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct MetaData {
        pub name: String,
        pub params: Vec<Param>,
        pub r#return: Return,
    }

    /// nest field
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Param {
        pub name: String,
        pub r#type: String,
        // pub dealloc: bool,
    }

    /// nest field
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Return {
        pub r#type: String,
        // pub dealloc: bool,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    /// 语言和对应的模板
    ///
    /// * `lang`: 语言
    /// * `lang_slug`: 语言
    /// * `code`: 模板
    pub struct CodeSnippet {
        lang: String,
        #[serde(alias = "langSlug")]
        lang_slug: String,
        code: String,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct TopicTags {
        name: String,
        slug: String,
        #[serde(alias = "translatedName")]
        translated_name: String,
    }
}
