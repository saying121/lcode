use serde::{Deserialize, Serialize};

use crate::entities::topic_tags;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct PbListData {
    #[serde(default)]
    pub data: DataInner,
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct DataInner {
    #[serde(default, alias = "problemsetQuestionList")]
    pub problemset_question_list: ProblemsetQuestionList,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ProblemsetQuestionList {
    // #[serde(default, alias = "hasMore")]
    // pub has_more:  bool, // for leetcode.cn
    #[serde(default)]
    pub total:     u32,
    #[serde(default)]
    pub questions: Vec<NewIndex>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct NewIndex {
    #[serde(default, alias = "titleSlug")]
    pub title_slug:           String,
    #[serde(default)]
    pub title:                String,
    #[serde(default, alias = "titleCn")]
    pub title_cn:             Option<String>,
    #[serde(default, alias = "isFavor")]
    pub is_favor:             bool,
    #[serde(default, alias = "frontendQuestionId")]
    pub frontend_question_id: String,
    #[serde(default, alias = "paidOnly")]
    pub paid_only:            bool,
    #[serde(default)]
    pub difficulty:           String,
    #[serde(default)]
    pub status:               Option<String>,
    #[serde(default, alias = "acRate")]
    pub ac_rate:              f64,
    #[serde(default, alias = "topicTags")]
    pub topic_tags:           Option<Vec<topic_tags::Model>>,
}
