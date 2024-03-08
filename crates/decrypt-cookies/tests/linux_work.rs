use decrypt_cookies::chromium::*;

#[tokio::test]
async fn it_work() {
    let a = linux::get_session_csrf("chrome".into(), "leetcode.cn")
        .await
        .unwrap();
    dbg!(a);
}
