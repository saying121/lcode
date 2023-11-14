use lcode::dao::query_topic_tags::*;
use miette::Result;

#[tokio::test]
async fn query() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let tags = Vec::from([
        "array".to_owned(),
        // "graph".to_owned(),
        "hash-table".to_owned(),
    ]);
    let res = query_by_topic(&tags, None).await?;
    for i in &res {
        println!("{}: {}", i.frontend_question_id, i.title_slug);
    }
    println!(r##"(| res.len() |) -> {:#?}"##, res.len());

    Ok(())
}
#[tokio::test]
async fn query_count() -> Result<()> {
    let a = query_status().await?;
    dbg!(a);

    Ok(())
}

#[tokio::test]
async fn query_all() -> Result<()> {
    let alltop: Vec<lcode::entities::topic_tags::Model> = query_all_topic().await?;
    println!(r##"(| alltop |) -> {:#?}"##, alltop.len());
    let all_new_index = query_all_new_index(None).await?;
    println!(r##"(| all_new_index |) -> {:#?}"##, all_new_index.len());
    Ok(())
}
