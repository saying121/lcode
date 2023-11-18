use lcode::{
    dao::{self, query_topic_tags::*},
    leetcode::IdSlug,
};
use miette::Result;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let tags = Vec::from([
        "array".to_owned(),
        // "graph".to_owned(),
        "hash-table".to_owned(),
    ]);
    let res = query_by_topic(&tags, None).await?;

    assert!(res
        .iter()
        .any(|v| { v.title_slug == "two-sum" }));

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_count() -> Result<()> {
    let a = query_status().await?;

    assert_eq!(a[0].0, "EASY");
    assert_eq!(a[1].0, "HARD");
    assert_eq!(a[2].0, "MEDIUM");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_all_topic_tags() -> Result<()> {
    let alltop: Vec<lcode::entities::topic_tags::Model> = query_all_topic().await?;
    let slug: Vec<String> = alltop
        .iter()
        .map(|v| v.topic_slug.to_owned())
        .collect();
    assert_eq!(
        slug,
        vec![
            "array",
            "hash-table",
            "recursion",
            "linked-list",
            "math",
            "string",
            "sliding-window",
            "binary-search",
            "divide-and-conquer",
            "dynamic-programming",
            "greedy",
            "two-pointers",
            "trie",
            "sorting",
            "backtracking",
            "stack",
            "heap-priority-queue",
            "merge-sort",
            "string-matching",
            "bit-manipulation",
            "matrix",
            "monotonic-stack",
            "simulation",
            "combinatorics",
            "memoization",
            "tree",
            "depth-first-search",
            "binary-tree",
            "binary-search-tree",
            "breadth-first-search",
            "union-find",
            "graph",
            "design",
            "doubly-linked-list",
            "geometry",
            "interactive",
            "bucket-sort",
            "radix-sort",
            "counting",
            "data-stream",
            "iterator",
            "database",
            "hash-function",
            "rolling-hash",
            "shell",
            "game-theory",
            "binary-indexed-tree",
            "prefix-sum",
            "ordered-set",
            "queue",
            "monotonic-queue",
            "bitmask",
            "number-theory",
            "topological-sort",
            "probability-and-statistics",
            "minimum-spanning-tree",
            "strongly-connected-component",
            "shortest-path",
            "brainteaser",
            "enumeration",
            "segment-tree",
            "quickselect",
            "rejection-sampling",
            "randomized",
            "reservoir-sampling",
            "suffix-array",
            "counting-sort",
            "line-sweep",
            "concurrency",
            "biconnected-component",
            "eulerian-circuit",
        ]
    );

    let all_new_index = query_all_new_index(None).await?;
    assert!(all_new_index.len() > 2900, "lose some question");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_question_work() -> Result<()> {
    let a = dao::get_question_index(IdSlug::Id(0)).await?;
    assert_eq!(a.len(), 0);

    let a = lcode::dao::get_question_index_exact(&IdSlug::Id(1)).await?;

    assert_eq!(a.question_id, 1);
    assert_eq!(a.question_title, "Two Sum");
    assert_eq!(a.question_title_slug, "two-sum");
    assert_eq!(a.frontend_question_id, "1");

    Ok(())
}
