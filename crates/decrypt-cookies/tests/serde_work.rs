use decrypt_cookies::Browser;

#[test]
fn browser_to_str() {
    let edge = Browser::Edge;
    let edge = toml::to_string(&edge);
    dbg!(&edge);
    let pat: Browser = toml::from_str("edge").unwrap();
    dbg!(pat);

    let pat = serde_json::to_string(&Browser::Edge).unwrap();
    dbg!(pat);
    // let pat:Browser = serde_json::from_str("edge").unwrap();
    // dbg!(pat);

    let pat = toml::to_string(&Browser::Edge).unwrap();
    dbg!(pat);
}
