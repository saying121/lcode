use std::fmt::Display;

use atoi::atoi;
use inquire::Select;
use miette::Result;
use nucleo::{
    pattern::{CaseMatching, Pattern},
    Config, Matcher,
};
use rayon::prelude::*;

use crate::{config::global::glob_user_config, dao};

pub async fn select_a_question() -> Result<u32> {
    let vc = dao::query_all_index().await?;

    let indexs = vc
        .into_iter()
        .map(|v| v.to_string())
        .collect();

    let a = Select::new("Select question ❓:", indexs)
        .with_formatter(&|v| format!("{:.10}", v.to_string()))
        .with_filter(&filter)
        .with_page_size(glob_user_config().page_size)
        .prompt()
        .unwrap_or_default();

    let bt: Vec<&str> = a.split('[').collect();
    let ids = bt
        .get(1)
        .copied()
        .unwrap_or_default();

    let res = atoi::<u32>(ids.as_bytes()).unwrap_or_default();

    Ok(res)
}

#[inline]
pub fn filter<T>(input: &str, _: &T, string_value: &str, _: usize) -> bool
where
    T: Display,
{
    use simsearch::SimSearch;
    let mut search_engine = SimSearch::new();
    search_engine.insert(string_value, string_value);
    let res = search_engine.search(input);

    res.contains(&string_value)
        || string_value
            .to_lowercase()
            .contains(&input.to_lowercase())
}

pub fn new_filter<T>(a: Vec<T>, pat: &str) -> Vec<(T, u32)>
where
    T: Display + Sized + Send + Sync,
{
    let a: Vec<String> = a
        .into_par_iter()
        .map(|v| v.to_string())
        .collect();

    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    let matches = Pattern::parse(pat, CaseMatching::Ignore).match_list(a, &mut matcher);

    let a = vec![];
    a
}
