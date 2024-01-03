use decrypt_cookies::{get_cookie, Browser};
use miette::{IntoDiagnostic, Result};
use secret_service::{EncryptionType, SecretService};

#[ignore = "need realy environment"]
#[tokio::test]
async fn get_cookie_work() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let leetcode_cn = "leetcode.cn";
    let leetcode_com = "leetcode.com";
    let edge = get_cookie(Browser::Edge, leetcode_cn)
        .await
        .unwrap_or_default();
    println!(r##"(| {} {leetcode_cn} |) -> {edge:#?}"##, Browser::Edge);
    let edge = get_cookie(Browser::Edge, leetcode_com)
        .await
        .unwrap_or_default();
    println!(r##"(| {} {leetcode_com} |) -> {edge:#?}"##, Browser::Edge,);

    let chrome = get_cookie(Browser::Chrome, leetcode_cn)
        .await
        .unwrap_or_default();
    println!(r##"(| chrome cn |) -> {:#?}"##, chrome);

    let ff = get_cookie(Browser::Firefox, leetcode_cn)
        .await
        .unwrap_or_default();
    println!(r##"(| ff cn |) -> {:#?}"##, ff);

    let librewolf = get_cookie(Browser::Librewolf, leetcode_cn)
        .await
        .unwrap_or_default();
    println!(r##"(| librewolf cn |) -> {:#?}"##, librewolf);

    Ok(())
}

#[ignore = "just inspect"]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn all_pass() -> Result<()> {
    // initialize secret service (dbus connection and encryption session)
    let ss = SecretService::connect(EncryptionType::Dh)
        .await
        .unwrap();
    // get default collection
    let collection = ss
        .get_default_collection()
        .await
        .unwrap();
    let coll = collection
        .get_all_items()
        .await
        .into_diagnostic()?;
    for i in coll {
        let lab = &i
            .get_label()
            .await
            .into_diagnostic()?;
        let res = i
            .get_secret()
            .await
            .into_diagnostic()?;
        let pass = String::from_utf8_lossy(&res).to_string();
        println!(r##"(| lab |) -> {}, (| pass |) -> {}"##, lab, pass);
    }

    Ok(())
}
