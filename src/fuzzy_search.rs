use atoi::atoi;
use inquire::Select;
use miette::Error;
use simsearch::SimSearch;

use crate::{config::global::global_user_config, storage::query_question};

pub async fn select_a_question() -> Result<u32, Error> {
    let user = global_user_config();

    // let rt = tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .into_diagnostic()?;
    // let vc = rt
    //     .block_on(query_question::query_all_index())
    //     .unwrap_or_default();
    let vc = query_question::query_all_index().await?;

    let indexs = vc
        .into_iter()
        .map(|v| v.to_string())
        .collect();

    let a = Select::new("Select question ❓:", indexs)
        .with_formatter(&|v| format!("{:.10}", v.to_string()))
        .with_filter(&filter)
        .with_page_size(user.page_size)
        .prompt()
        .unwrap();

    let mut bt = a.chars();
    bt.next();
    bt.next();
    let ids: String = bt.collect();

    let res = atoi::<u32>(ids.as_bytes()).unwrap_or_default();
    Ok(res)
}

fn filter<'a, T>(input: &str, _: &T, string_value: &str, _: usize) -> bool
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
