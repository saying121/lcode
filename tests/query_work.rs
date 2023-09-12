use lcode::dao::query_topic_tags::get_all_topic;

#[tokio::test]
async fn query() -> miette::Result<()> {
    use lcode::dao::query_topic_tags::find_topic;
    let res = find_topic(vec!["array", "dynamic-programming"]).await?;
    println!(r##"(| res |) -> {:#?}"##, res);
    Ok(())
}


#[tokio::test]
async fn query_alltop() -> miette::Result<()> {
    let alltop = get_all_topic().await?;
    println!(r##"(| alltop |) -> {:#?}"##, alltop);
    Ok(())
}
