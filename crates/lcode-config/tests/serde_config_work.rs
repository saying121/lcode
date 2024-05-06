use lcode_config::{
    config::{read_config, user_nested::Suffix},
    global::G_USER_CONFIG,
};
use miette::Result;

#[test]
fn serde_conf_work() -> Result<()> {
    read_config::gen_config(Suffix::Cn)?;
    // let a = read_config::get_user_conf()?;
    // println!(r##"(| a |) -> {:#?}"##, a);
    // let a = &USER_CONFIG.get_suffix();
    // dbg!(a);
    let a = toml::to_string(&*G_USER_CONFIG).unwrap();
    println!("{}", a);

    Ok(())
}
