use std::collections::HashMap;

use crate::{config::global::glob_user_config, Json};

pub struct QueryProblemSet {
    pub json: Json,
}

impl QueryProblemSet {
    pub fn new(skip: u32) -> Self {
        const GRQL_CN: &str =
            include_str!("../../graphqls/problemsetQuestionList_cn.graphql");
        const GRQL_COM: &str =
            include_str!("../../graphqls/problemsetQuestionList_com.graphql");

        let skip = skip.to_string();

        let (graphql, var) = match glob_user_config()
            .config
            .url_suffix
            .as_str()
        {
            "cn" => (
                GRQL_CN,
                r#"{"skip":$skip,"limit":100,"filters":{}}"#.replace("$skip", &skip),
            ),
            "com" => (
                GRQL_COM,
                r#"{"categorySlug":"","skip":$skip,"limit":100,"filters":{}}"#
                    .replace("$skip", &skip),
            ),
            _ => (
                GRQL_COM,
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
        const GRQL_CN: &str = include_str!("../../graphqls/get_list_count_cn.graphql");
        const GRQL_COM: &str = include_str!("../../graphqls/get_list_count_com.graphql");

        let (graphql, var) = match glob_user_config()
            .config
            .url_suffix
            .as_str()
        {
            "cn" => (GRQL_CN, r#"{"skip":0,"limit":0,"filters":{}}"#),
            "com" => (
                GRQL_COM,
                r#"{"categorySlug":"","skip":0,"limit":0,"filters":{}}"#,
            ),
            _ => (
                GRQL_COM,
                r#"{"categorySlug":"","skip":0,"limit":0,"filters":{}}"#,
            ),
        };
        let mut json: Json = HashMap::new();
        json.insert("query", graphql.to_owned());

        json.insert("variables", var.to_owned());
        json.insert("operationName", "problemsetQuestionList".to_owned());
        Self { json }
    }
}

pub(super) fn init_qs_detail_grql(qs_title_slug: &str) -> Json {
    const GRQL: &str = include_str!("../../graphqls/getQuestion_detail.graphql");

    let mut json: Json = HashMap::new();
    json.insert("query", GRQL.to_owned());

    json.insert(
        "variables",
        r#"{"titleSlug": "$titleSlug"}"#.replace("$titleSlug", qs_title_slug),
    );
    json.insert("operationName", "getQuestion".to_owned());
    json
}

pub(super) fn init_subit_list_grql(qs_title_slug: &str) -> Json {
    const GRQL: &str = include_str!("../../graphqls/submissionList.graphql");

    let mut json: Json = HashMap::new();
    json.insert("query", GRQL.to_owned());
    json.insert(
            "variables",
            r#"{"questionSlug":"$Slug", "offset":0,"limit":$num,"lastKey":null,"status":null}"#
                .replace("$Slug", qs_title_slug)
                .replace("$num", &glob_user_config().config.num_sublist.to_string()),
        );
    json.insert("operationName", "submissionList".to_owned());
    json
}
