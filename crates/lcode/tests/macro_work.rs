use lcode_config::config::user_nest::Rust;
use pretty_assertions::assert_eq;

#[test]
fn macro_langs() {
    let rs = Rust::default();
    dbg!(rs.return_info());
    assert_eq!(&rs.start, "//start/");
    assert_eq!(&rs.end, "//end/");
    assert_eq!(&rs.inject_start, "struct Solution;\n");
    assert_eq!(
        &rs.inject_end,
        "\nfn main() {\n    println!(\"{:#?}\", Solution::function());\n}"
    );
}
