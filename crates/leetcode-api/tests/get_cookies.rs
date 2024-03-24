use leetcode_api::leetcode::headers::Headers;

#[tokio::test]
async fn get_cookies_work() {
    let var = Headers::build("leetcode.cn")
        .await
        .unwrap();
    dbg!(var);
}
