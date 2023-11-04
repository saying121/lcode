use lcode::cookies::get_cookie;
use miette::{IntoDiagnostic, Result};

#[tokio::test]
async fn get_cookie_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let edge = get_cookie("edge").await?;
    println!(r##"(| edge |) -> {:#?}"##, edge);

    let chrome = get_cookie("chrome").await?;
    println!(r##"(| chrome |) -> {:#?}"##, chrome);

    let ff = get_cookie("firefox").await?;
    println!(r##"(| ff |) -> {:#?}"##, ff);

    let librewolf = get_cookie("librewolf").await?;
    println!(r##"(| librewolf |) -> {:#?}"##, librewolf);

    Ok(())
}

#[ignore = "just inspect"]
#[tokio::test]
async fn all_pass() -> Result<()> {
    // dbus_session.
    use secret_service::EncryptionType;
    use secret_service::SecretService;
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
