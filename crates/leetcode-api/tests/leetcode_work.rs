use leetcode_api::{glob_leetcode, leetcode::IdSlug};
use pretty_assertions::assert_eq;

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_qs_detail_work() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let lcode = glob_leetcode().await;
    let question = lcode
        .get_qs_detail(
            IdSlug::Slug("find-smallest-common-element-in-all-rows".to_owned()),
            false,
            true,
        )
        .await
        .unwrap();
    assert_eq!(
        &question.qs_slug.unwrap(),
        "find-smallest-common-element-in-all-rows"
    );
    assert_eq!(
        &question.example_testcases,
        "[[1,2,3,4,5],[2,4,5,8,10],[3,5,7,9,11],[1,3,5,7,9]]\n[[1,2,3],[2,3,4],[2,3,5]]"
    );
    assert_eq!(
        &question.sample_test_case,
        "[[1,2,3,4,5],[2,4,5,8,10],[3,5,7,9,11],[1,3,5,7,9]]"
    );
    assert_eq!(&question.question_id, "1143");
    assert_eq!(
        &question.question_title.unwrap(),
        "Find Smallest Common Element in All Rows"
    );
    assert_eq!(&question.title, "Find Smallest Common Element in All Rows");

    let question = lcode
        .get_qs_detail(IdSlug::Slug("two-sum".to_owned()), false, true)
        .await
        .unwrap();
    assert_eq!(&question.question_id, "1");
    assert_eq!(&question.title, "Two Sum");
    assert_eq!(&question.qs_slug.unwrap(), "two-sum");
    assert_eq!(&question.question_title.unwrap(), "Two Sum");

    let question = lcode
        .get_qs_detail(IdSlug::Id(195), false, true)
        .await
        .unwrap();
    assert_eq!(&question.qs_slug.unwrap(), "tenth-line");
    assert_eq!(&question.question_id, "195");
    assert_eq!(&question.question_title.unwrap(), "Tenth Line");
    assert_eq!(&question.title, "Tenth Line");
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_user_code_work() {
    let id = IdSlug::Id(108);
    glob_leetcode()
        .await
        .get_qs_detail(id.clone(), false, true)
        .await
        .unwrap();

    let a = glob_leetcode()
        .await
        .get_user_code(id)
        .await
        .unwrap();

    assert!(!a.0.is_empty());
    assert_eq!(&a.1, "[-10,-3,0,5,9]\n[1,3]");
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_qs_detail_none() {
    assert!(
        glob_leetcode()
            .await
            .get_qs_detail(IdSlug::Id(0), false, true)
            .await
            .is_err()
    );
}
