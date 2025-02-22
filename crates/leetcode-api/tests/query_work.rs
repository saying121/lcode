#![expect(clippy::panic_in_result_fn, reason = "tests")]

use lcode_config::{config::user_nested::Suffix, global::G_USER_CONFIG};
use leetcode_api::{dao::query::*, entities::topic_tags};
use miette::Result;

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_base() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let tags = Vec::from([
        "array".to_owned(),
        // "graph".to_owned(),
        "hash-table".to_owned(),
    ]);
    let res = Query::query_by_topic(&tags, None).await?;
    // for ele in &res {
    //     eprintln!("{}", &ele.title_slug);
    // }

    let mut iter = res.iter();
    assert!(iter.next().unwrap().title_slug == "3sum-with-multiplicity");
    assert!(iter.next().unwrap().title_slug == "4sum-ii");
    assert!(iter.next().unwrap().title_slug == "accounts-merge");

    assert!(
        res.iter()
            .any(|v| { v.title_slug == "two-sum" })
    );

    Ok(())
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_count() -> Result<()> {
    let a = Query::query_status().await?;

    if G_USER_CONFIG.config.url_suffix == Suffix::Cn {
        assert_eq!(a[0].diff, "EASY");
        assert_eq!(a[1].diff, "HARD");
        assert_eq!(a[2].diff, "MEDIUM");
    }
    else {
        assert_eq!(a[0].diff, "Easy");
        assert_eq!(a[1].diff, "Hard");
        assert_eq!(a[2].diff, "Medium");
    }

    Ok(())
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_all_topic_tags() -> Result<()> {
    let alltop: Vec<topic_tags::Model> = Query::query_all_topic().await?;

    assert!(alltop.len() > 70);

    let all_new_index = Query::query_all_new_index(None).await?;
    assert!(all_new_index.len() > 2900, "lose some question");

    Ok(())
}
