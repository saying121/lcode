use std::ops::Deref;

use lcode_config::config::{global::G_USER_CONFIG, user_nest::Suffix};

use crate::Json;

pub struct QueryProblemSet(pub Json);

impl Deref for QueryProblemSet {
    type Target = Json;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl QueryProblemSet {
    pub fn new(skip: u32) -> Self {
        const GRQL_CN: &str = include_str!("../../graphqls/problemsetQuestionList_cn.graphql");
        const GRQL_COM: &str = include_str!("../../graphqls/problemsetQuestionList_com.graphql");

        let skip = skip.to_string();

        let (graphql, var) = match G_USER_CONFIG.config.url_suffix {
            Suffix::Cn => (
                GRQL_CN,
                r#"{"skip":$skip,"limit":100,"filters":{}}"#.replace("$skip", &skip),
            ),
            Suffix::Com => (
                GRQL_COM,
                r#"{"categorySlug":"","skip":$skip,"limit":100,"filters":{}}"#
                    .replace("$skip", &skip),
            ),
        };

        let mut json: Json = Json::with_capacity(3);
        json.insert("query", graphql.to_owned());

        json.insert("variables", var);
        json.insert("operationName", "problemsetQuestionList".to_owned());
        Self(json)
    }

    pub fn get_count() -> Self {
        const GRQL_CN: &str = include_str!("../../graphqls/get_list_count_cn.graphql");
        const GRQL_COM: &str = include_str!("../../graphqls/problemsetQuestionList_com.graphql");

        let (graphql, var) = match G_USER_CONFIG.config.url_suffix {
            Suffix::Cn => (GRQL_CN, r#"{"skip":0,"limit":0,"filters":{}}"#),
            Suffix::Com => (
                GRQL_COM,
                r#"{"categorySlug":"","skip":0,"limit":1,"filters":{}}"#,
            ),
        };
        let mut json: Json = Json::with_capacity(3);
        json.insert("query", graphql.to_owned());

        json.insert("variables", var.to_owned());
        json.insert("operationName", "problemsetQuestionList".to_owned());
        Self(json)
    }
}

pub(super) fn init_qs_detail_grql(qs_title_slug: &str) -> Json {
    const GRQL: &str = include_str!("../../graphqls/getQuestion_detail.graphql");

    let mut json: Json = Json::with_capacity(3);
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

    let mut json: Json = Json::with_capacity(3);
    json.insert("query", GRQL.to_owned());
    json.insert(
        "variables",
        r#"{"questionSlug":"$Slug", "offset":0,"limit":$num,"lastKey":null,"status":null}"#
            .replace("$Slug", qs_title_slug)
            .replace(
                "$num",
                &G_USER_CONFIG
                    .config
                    .num_sublist
                    .to_string(),
            ),
    );
    json.insert("operationName", "submissionList".to_owned());
    json
}

pub(super) fn daily_checkin_grql() -> Json {
    const DAILY_CHECKIN: &str = include_str!("../../graphqls/dailyCheckin.graphql");
    let mut json: Json = Json::with_capacity(3);
    json.insert("query", DAILY_CHECKIN.to_owned());
    json.insert("variables", "{}".to_owned());
    json.insert("operationName", "dailyCheckin".to_owned());
    json
}

pub(super) fn global_data_grql() -> Json {
    const GRQL: &str = include_str!("../../graphqls/globalData_user_info.graphql");
    let mut json: Json = Json::with_capacity(3);
    json.insert("query", GRQL.to_owned());
    json.insert("variables", "{}".to_owned());
    json.insert("operationName", "globalData".to_owned());
    json
}

pub(super) fn pass_status_grql(user_slug: &str) -> Json {
    const PASS_GRQL_CN: &str = include_str!("../../graphqls/pass_cn.graphql");
    const PASS_GRQL_COM: &str = include_str!("../../graphqls/pass_com.graphql");

    let pat = match G_USER_CONFIG.config.url_suffix {
        Suffix::Cn => PASS_GRQL_CN,
        Suffix::Com => PASS_GRQL_COM,
    };

    let mut json: Json = Json::with_capacity(3);
    json.insert("query", pat.to_owned());
    json.insert(
        "variables",
        r#"{"userSlug":"$userSlug"}"#.replace("$userSlug", user_slug),
    );
    json.insert("operationName", "userSessionProgress".to_owned());
    json
}
