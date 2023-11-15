use std::collections::HashMap;

use crate::config::global::glob_user_config;

use super::Json;

pub struct QueryProblemSet {
    pub json: Json,
}

impl QueryProblemSet {
    pub fn new(skip: u64) -> Self {
        let skip = skip.to_string();

        let (graphql, var) = match glob_user_config()
            .url_suffix
            .as_str()
        {
            "cn" => (
                include_str!("../../graphqls/problemsetQuestionList_cn.graphql"),
                r#"{"skip":$skip,"limit":100,"filters":{}}"#.replace("$skip", &skip),
            ),
            "com" => (
                include_str!("../../graphqls/problemsetQuestionList_com.graphql"),
                r#"{"categorySlug":"","skip":$skip,"limit":100,"filters":{}}"#
                    .replace("$skip", &skip),
            ),
            _ => (
                include_str!("../../graphqls/problemsetQuestionList_com.graphql"),
                r#"{"categorySlug":"","skip":$skip,"limit":100,"filters":{}}"#
                    .replace("$skip", &skip),
            ),
        };

        let mut json: Json = HashMap::new();
        json.insert("query", graphql.to_owned());

        json.insert("variables", var);
        json.insert("operationName", "problemsetQuestionList".to_owned());
        Self { json }
    }

    pub fn get_count() -> Self {
        let (graphql, var) = match glob_user_config()
            .url_suffix
            .as_str()
        {
            "cn" => (
                include_str!("../../graphqls/get_list_count_cn.graphql").to_owned(),
                r#"{"skip":0,"limit":0,"filters":{}}"#,
            ),
            "com" => (
                include_str!("../../graphqls/get_list_count_com.graphql").to_owned(),
                r#"{"categorySlug":"","skip":0,"limit":0,"filters":{}}"#,
            ),
            _ => (
                include_str!("../../graphqls/get_list_count_com.graphql").to_owned(),
                r#"{"categorySlug":"","skip":0,"limit":0,"filters":{}}"#,
            ),
        };
        let mut json: Json = HashMap::new();
        json.insert("query", graphql);

        json.insert("variables", var.to_owned());
        json.insert("operationName", "problemsetQuestionList".to_owned());
        Self { json }
    }
}

pub(super) fn init_qs_detail_grql(qs_title_slug: &str) -> Json {
    let grql = include_str!("../../graphqls/getQuestion_detail.graphql");

    let mut json: Json = HashMap::new();
    json.insert("query", grql.to_owned());

    json.insert(
        "variables",
        r#"{"titleSlug": "$titleSlug"}"#.replace("$titleSlug", qs_title_slug),
    );
    json.insert("operationName", "getQuestion".to_owned());
    json
}

pub(super) fn init_subit_list_grql(qs_title_slug: &str) -> Json {
    let grql = include_str!("../../graphqls/submissionList.graphql");

    let mut json: Json = HashMap::new();
    json.insert("query", grql.to_owned());
    json.insert(
            "variables",
            r#"{"questionSlug":"$Slug", "offset":0,"limit":$num,"lastKey":null,"status":null}"#
                .replace("$Slug", qs_title_slug)
                .replace("$num", &glob_user_config().num_sublist.to_string()),
        );
    json.insert("operationName", "submissionList".to_owned());
    json
}
