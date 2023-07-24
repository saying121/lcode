use crate::config;

use super::question_detail::Question;
use miette::{miette, Error, IntoDiagnostic};
use serde_json::Value;
use tokio::fs::read_to_string;

/// 解析问题json
///
/// * `v`: serde_json::Value
pub fn parser_question(v: Value) -> Question {
    let def_v = Value::default();

    let temp = "content";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let content = match v.get(temp) {
        Some(it) => Some(it.to_string()),
        None => None,
    };

    let temp = "questionTitle";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let question_title = match v.get(temp) {
        Some(it) => Some(it.to_string()),
        None => None,
    };

    let temp = "translatedTitle";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let translated_title = match v.get(temp) {
        Some(it) => Some(it.to_string()),
        None => None,
    };

    let temp = "translatedContent";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let translated_content = match v.get(temp) {
        Some(it) => Some(it.to_string()),
        None => None,
    };

    let temp = "stats";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let stats = serde_json::from_str(
        v.get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
    )
    .unwrap_or_default();

    let temp = "sampleTestCase";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let sample_test_case = v
        .get(temp)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let temp = "exampleTestcases";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let example_testcases = v
        .get(temp)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let temp = "metaData";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let meta_data = serde_json::from_str(
        v.get(temp)
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
    )
    .unwrap_or_default();

    let temp = "hints";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let hints = serde_json::from_value(
        v.get(temp)
            .unwrap_or(&def_v)
            .clone(),
    )
    .unwrap_or_default();

    let temp = "mysqlSchemas";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let mysql_schemas = serde_json::from_value(
        v.get(temp)
            .unwrap_or(&def_v)
            .clone(),
    )
    .unwrap_or_default();

    let temp = "dataSchemas";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let data_schemas = serde_json::from_value(
        v.get(temp)
            .unwrap_or(&def_v)
            .clone(),
    )
    .unwrap_or_default();

    let temp = "questionId";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let question_id = v
        .get(temp)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let temp = "isPaidOnly";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let is_paid_only = v
        .get(temp)
        .and_then(|v| v.as_bool())
        .unwrap_or_default();

    let temp = "codeSnippets";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let code_snippets = serde_json::from_value(
        v.get(temp)
            .unwrap_or(&def_v)
            .clone(),
    )
    .unwrap_or_default();

    let temp = "title";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let title = v
        .get(temp)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let temp = "difficulty";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
    let difficulty = v
        .get(temp)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let temp = "topicTags";
    #[cfg(debug_assertions)]
    println!("debug:Deserialize {}", temp);
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

/// 通过题目id获取查询字符串
///
/// * `category`: 题目类别["algorithms", "concurrency", "database", "shell"]其中一个
/// * `id`: 题目id
pub async fn from_id_get_slug(
    category: String,
    id: usize,
) -> Result<String, Error> {
    let df_v = Value::default();
    let cache_path = config::init_cache_dir();

    let mut file = cache_path.clone();
    file.push(category + "/" + id.to_string().as_str() + ".json");
    let file_str = read_to_string(file)
        .await
        .map_err(|_e| miette!("don't exist the question"))?;

    let v: Value = serde_json::from_str(&file_str).into_diagnostic()?;

    Ok(v.get("stat")
        .unwrap_or(&df_v)
        .get("question__title_slug")
        .unwrap_or(&df_v)
        .to_string())
}

pub async fn from_slug_get_id(
    category: String,
    slug: &str,
) -> Result<String, Error> {
    let df_v = Value::default();
    let cache_path = config::init_cache_dir();

    let mut file = cache_path.clone();
    file.push(category + "/" + slug + ".json");
    let file_str = read_to_string(file)
        .await
        .into_diagnostic()?;

    let v: Value = serde_json::from_str(&file_str).into_diagnostic()?;

    Ok(v.get("stat")
        .unwrap_or(&df_v)
        .get("question__title_slug")
        .unwrap_or(&df_v)
        .to_string())
}
