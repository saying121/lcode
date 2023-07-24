pub mod parser;
use self::question_detail::Question;
use crate::config::{self, Config, User};
use colored::Colorize;
use miette::{Error, IntoDiagnostic};
use parser::*;
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde_json::Value;
use std::{collections::HashMap, sync::OnceLock, time::Duration};
use tokio::{
    fs::{create_dir_all, read_to_string, OpenOptions},
    io::AsyncWriteExt,
};

pub enum Index {
    Id(u64),
    Slug(String),
}

// static STATIC: Type = init;

static QS_DETAIL_GRAPHQL: OnceLock<Vec<&str>> = OnceLock::new();
fn init_qs_dt_grql() -> Vec<&'static str> {
    QS_DETAIL_GRAPHQL
        .get_or_init(|| {
            vec![
                "query getQuestion($titleSlug: String!) {",
                "    question(titleSlug: $titleSlug) {",
                "        content",          // 题目描述
                "        stats",            // 题目通过/提交.etc状态
                "        sampleTestCase",   // 测试用例
                "        exampleTestcases", // 例子
                "        metaData",
                "        translatedTitle", // 翻译后的标题
                "        translatedContent", // 翻译后的题目描述, 示例，提示，进阶(是个html)
                "        hints",             // 提示
                "        mysqlSchemas",
                "        dataSchemas",
                "        questionId",    // 问题 id
                "        questionTitle", // 标题
                "        isPaidOnly",    // 是否仅付费用户
                "        codeSnippets {",
                "            lang",
                "            langSlug",
                "            code", // 获取模板
                "        }",
                "        title",
                "        isPaidOnly",
                "        difficulty",
                "        topicTags {",
                "            name", // 类别名字
                "            slug",
                "            translatedName", // 中文名字
                "        }",
                "    }",
                "}",
            ]
        })
        .to_vec()
}

mod question_detail;

pub type Json = HashMap<&'static str, String>;

pub struct LeetCode {
    pub client: Client,
    pub headers: HeaderMap,
}

impl LeetCode {
    /// Create a LeetCode instance and initialize some variables
    pub async fn new() -> Result<Self, Error> {
        let config = Config::new().await?;

        let client = ClientBuilder::new()
            .gzip(true)
            .connect_timeout(Duration::from_secs(30))
            .build()
            .into_diagnostic()?;

        Ok(LeetCode {
            client,
            headers: config.headers,
        })
    }

    /// Get the details of the problem, and if it's in the cache, use it
    ///
    /// * `category`: category of the problem
    /// * `id`: id of the problem
    /// * `force`: when true, the cache will be re-fetched
    pub async fn get_problem_detail(
        &self,
        category: String,
        id: usize,
        force: bool,
    ) -> Result<Question, Error> {
        let mut cache_pb_details_path = config::init_cache_detail_dir().clone();
        cache_pb_details_path.push(id.to_string() + ".json");

        #[cfg(debug_assertions)]
        println!("debug:mkdir -> {:?}", cache_pb_details_path);
        create_dir_all(
            cache_pb_details_path
                .parent()
                .unwrap(),
        )
        .await
        .into_diagnostic()?;

        #[cfg(debug_assertions)]
        println!(r##"要查询的问题"##);
        let slug = parser::from_id_get_slug(category, id).await?;
        let slug = slug.trim_matches('"');

        let mut _pb_data = Value::default();
        let mut _res_qs = Question::default();

        #[cfg(debug_assertions)]
        println!(r##"start"##);
        if force || !cache_pb_details_path.exists() {
            #[cfg(debug_assertions)]
            println!(r##"bf"##);
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&cache_pb_details_path)
                .await
                .into_diagnostic()?;

            let user = User::default();
            let url = user.graphql.to_string();
            let mut json: Json = HashMap::new();
            json.insert("query", init_qs_dt_grql().join("\n"));

            json.insert(
                "variables",
                r#"{"titleSlug": "$titleSlug"}"#.replace("$titleSlug", slug),
            );
            json.insert("operationName", "getQuestion".to_string());

            #[cfg(debug_assertions)]
            println!(r##"获取问题bf"##);
            let req = self
                .client
                .post(url)
                .json(&json)
                .headers(self.headers.clone())
                .send()
                .await
                .into_diagnostic()?;
            #[cfg(debug_assertions)]
            println!(r##"获取问题 end"##);
            let pb_json: Value = req
                .json()
                .await
                .into_diagnostic()?;

            _pb_data = pb_json
                .get("data")
                .unwrap_or(&Value::default())
                .get("question")
                .unwrap_or(&Value::default())
                .clone();

            #[cfg(debug_assertions)]
            println!(r##"(| debug:获取的问题json  |) -> {:#?}"##, _pb_data);

            _res_qs = parser_question(_pb_data.clone());

            let question_string =
                serde_json::to_string(&_res_qs).unwrap_or("".to_string());

            file.write(question_string.as_bytes())
                .await
                .into_diagnostic()?;
        } else {
            let string = read_to_string(&cache_pb_details_path)
                .await
                .into_diagnostic()?;
            _res_qs = serde_json::from_str(&string).unwrap_or_else(|_v| {
                eprintln!(
                    "{}",
                    "cache broken please redownload or force download the quesion,now use default"
                        .red()
                );
                Question::default()
            });
        }

        Ok(_res_qs)
    }
}
