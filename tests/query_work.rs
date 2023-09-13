use lcode::dao::query_topic_tags::query_all_topic;

#[tokio::test]
async fn query() -> miette::Result<()> {
    use lcode::dao::query_topic_tags::query_by_topic;
    // let res = find_topic(vec!["array", "dynamic-programming"]).await?;
    // println!(r##"(| res |) -> {:#?}"##, res);
    let res = query_by_topic([]).await?;
    println!(r##"(| res |) -> {:#?}"##, res);
    println!(r##"(| res.len() |) -> {:#?}"##, res.len());
    Ok(())
}

#[tokio::test]
async fn query_alltop() -> miette::Result<()> {
    let alltop: Vec<lcode::entities::topic_tags::Model> = query_all_topic().await?;
    println!(r##"(| alltop |) -> {:#?}"##, alltop);
    Ok(())
}
