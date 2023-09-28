use atoi::atoi;
use inquire::Select;
use miette::Error;
use simsearch::SimSearch;

use crate::{config::global::glob_user_config, dao};

pub async fn select_a_question() -> Result<u32, Error> {
    let user = glob_user_config();

    let vc = dao::query_all_index().await?;

    let indexs = vc
        .into_iter()
        .map(|v| v.to_string())
        .collect();

    let a = Select::new("Select question ❓:", indexs)
        .with_formatter(&|v| format!("{:.10}", v.to_string()))
        .with_filter(&filter)
        .with_page_size(user.page_size)
        .prompt()
        .unwrap_or_default();

    let bt: Vec<&str> = a.split('[').collect();
    let ids = bt
        .get(1)
        .cloned()
        .unwrap_or_default();

    let res = atoi::<u32>(ids.as_bytes()).unwrap_or_default();

    Ok(res)
}

#[inline]
pub fn filter<T>(input: &str, _: &T, string_value: &str, _: usize) -> bool
where
    T: std::fmt::Display,
{
    let mut search_engine = SimSearch::new();
    search_engine.insert(string_value, string_value);
    let res = search_engine.search(input);

    res.contains(&string_value)
        || string_value
            .to_lowercase()
            .contains(&input.to_lowercase())
}
