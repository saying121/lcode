use leetcode_api::leetcode::headers::Headers;

#[tokio::test]
async fn build_header_work() {
    let var = Headers::build("leetcode.cn")
        .await
        .unwrap();
    dbg!(var);
}
