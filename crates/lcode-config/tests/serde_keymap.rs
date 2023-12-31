use lcode_config::keymap::TuiKeyMap;

#[test]
fn keymap_ser() {
    let a = TuiKeyMap::default();
    let a = toml::to_string(&a).unwrap();
    println!("{}", a);
}
