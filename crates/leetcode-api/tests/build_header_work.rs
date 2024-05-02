use leetcode_api::leetcode::headers::Headers;

#[ignore]
#[tokio::test]
async fn build_header_work() {
    let var = Headers::build("leetcode.cn")
        .await
        .unwrap();
    dbg!(var);
}
