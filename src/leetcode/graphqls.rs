use std::{collections::HashMap, sync::OnceLock};

use crate::config::global::glob_user_config;

pub type Json = HashMap<&'static str, String>;

pub struct QueryProblemSet {
    pub json: Json,
}

impl QueryProblemSet {
    pub fn new(skip: u64) -> Self {
        let (graphql, var) = match glob_user_config()
            .url_suffix
            .as_str()
        {
            "cn" => (
                init_pbsetlist_grql_cn().join("\n"),
                r#"{"skip":$skip,"limit":100,"filters":{}}"#
                    .replace("$skip", skip.to_string().as_str()),
            ),
            "com" => (
                init_pbsetlist_grql_com().join("\n"),
                r#"{"categorySlug":"","skip":$skip,"limit":100,"filters":{}}"#
                    .replace("$skip", skip.to_string().as_str()),
            ),
            _ => (
                init_pbsetlist_grql_com().join("\n"),
                r#"{"categorySlug":"","skip":$skip,"limit":100,"filters":{}}"#
                    .replace("$skip", skip.to_string().as_str()),
            ),
        };
        // Self { graphql, variables }
        let mut json: Json = HashMap::new();
        json.insert("query", graphql);

        json.insert("variables", var);
        json.insert("operationName", "problemsetQuestionList".to_string());
        Self { json }
    }
}
// # ","variables":{"categorySlug":"","skip":0,"limit":50,"filters":{}},"operationName":"problemsetQuestionList"}' \

pub(super) static PROBLEM_SET_QUESTION_LIST_CN: OnceLock<Vec<&str>> = OnceLock::new();
// ","variables":{"skip":0,"limit":4000,"filters":{}},"operationName":"problemsetQuestionList"}' \
pub(super) fn init_pbsetlist_grql_cn() -> Vec<&'static str> {
    PROBLEM_SET_QUESTION_LIST_CN
        .get_or_init(|| {
            vec![
            "query problemsetQuestionList(",
            "  $limit: Int",
            "  $skip: Int",
            "  $filters: QuestionListFilterInput",
            ") {",
            "  problemsetQuestionList(limit: $limit, skip: $skip, filters: $filters) {",
            // "    hasMore",
            "    total",
            "    questions {",
            "      acRate",
            "      difficulty",
            "      freqBar",
            "      frontendQuestionId",
            "      isFavor",
            "      paidOnly",
            "      solutionNum",
            "      status",
            "      title",
            "      titleCn",
            "      titleSlug",
            "      topicTags {",
            "        name",
            "        nameTranslated",
            "        id",
            "        slug",
            "      }",
            "    }",
            "  }",
            "}",
        ]
        })
        .to_vec()
}
pub(super) static PROBLEM_SET_QUESTION_LIST_COM: OnceLock<Vec<&str>> = OnceLock::new();
// ","variables":{"skip":0,"limit":4000,"filters":{}},"operationName":"problemsetQuestionList"}' \
pub(super) fn init_pbsetlist_grql_com() -> Vec<&'static str> {
    PROBLEM_SET_QUESTION_LIST_COM
        .get_or_init(|| {
            vec![
                "query problemsetQuestionList(",
                "  $categorySlug: String",
                "  $limit: Int",
                "  $skip: Int",
                "  $filters: QuestionListFilterInput",
                ") {",
                "  problemsetQuestionList: questionList(",
                "    categorySlug: $categorySlug",
                "    limit: $limit",
                "    skip: $skip",
                "    filters: $filters",
                "  ) {",
                "    total: totalNum",
                "    questions: data {",
                "      acRate",
                "      difficulty",
                "      freqBar",
                "      frontendQuestionId: questionFrontendId",
                "      isFavor",
                "      paidOnly: isPaidOnly",
                "      status",
                "      title",
                "      titleSlug",
                "      topicTags {",
                "        name",
                "        id",
                "        slug",
                "      }",
                "    }",
                "  }",
                "}",
            ]
        })
        .to_vec()
}

pub(super) static QS_DETAIL_GRAPHQL: OnceLock<Vec<&str>> = OnceLock::new();
pub(super) fn init_qs_dt_grql() -> Vec<&'static str> {
    QS_DETAIL_GRAPHQL
        .get_or_init(|| {
            vec![
                "query getQuestion($titleSlug: String!) {",
                "  question(titleSlug: $titleSlug) {",
                "    content",          // question content
                "    stats", // question pass submit status 题目通过/提交.etc状态
                "    sampleTestCase", // test case
                "    exampleTestcases", // example
                "    metaData",
                "    translatedTitle",   // translate
                "    translatedContent", // translate
                "    hints",
                "    mysqlSchemas",
                "    dataSchemas",
                "    questionId",
                "    questionTitle",
                "    isPaidOnly",
                "    codeSnippets {",
                "      lang",
                "      langSlug",
                "      code", // question template
                "    }",
                "    title",
                "    isPaidOnly",
                "    difficulty",
                "    topicTags {",
                "      name", // category name
                "      slug",
                "      translatedName", // translate
                "    }",
                "  }",
                "}",
            ]
        })
        .to_vec()
}

pub(super) static QUERY_SUBMISSION_LIST_GRAPHQL: OnceLock<Vec<&str>> = OnceLock::new();
pub(super) fn init_subit_list_grql() -> Vec<&'static str> {
    QUERY_SUBMISSION_LIST_GRAPHQL
        .get_or_init(|| {
            vec![
                "query submissionList(",
                "  $offset: Int!",
                "  $limit: Int!",
                "  $lastKey: String",
                "  $questionSlug: String!",
                "  $lang: String",
                "  $status: SubmissionStatusEnum",
                ") {",
                "  submissionList(",
                "    offset: $offset",
                "    limit: $limit",
                "    lastKey: $lastKey",
                "    questionSlug: $questionSlug",
                "    lang: $lang",
                "    status: $status",
                "  ) {",
                "    lastKey",
                "    hasNext",
                "    submissions {",
                "      id",
                "      title",
                "      status",
                "      statusDisplay",
                "      lang",
                "      langName: langVerboseName",
                "      runtime",
                "      timestamp",
                "      url",
                "      isPending",
                "      memory",
                "      submissionComment {",
                "        comment",
                "      } ",
                "    } ",
                "  } ",
                "}",
            ]
        })
        .to_vec()
}
