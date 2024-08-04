use crossterm::event::KeyCode;
use key_parse::keymap::{Key, Keys};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Test {
    keys: Keys,
}

#[test]
fn feature() {
    let strs = "keys = \"<C->\"\n";
    let keys: Test = toml::from_str(strs).unwrap();
    dbg!(keys);
}

#[test]
fn serde_tab() {
    let key = Test {
        keys: Keys(vec![Key {
            code: KeyCode::Tab,
            ..Default::default()
        }]),
    };
    let strs = toml::to_string(&key).unwrap();

    let right = "keys = \"<Tab>\"\n";
    let keys: Test = toml::from_str(&strs).unwrap();

    assert_eq!(strs, right);
    assert_eq!(key, keys);

    let key = Test {
        keys: Keys(vec![Key {
            code: KeyCode::Esc,
            ..Default::default()
        }]),
    };
    let strs = toml::to_string(&key).unwrap();

    let right = "keys = \"<Esc>\"\n";
    let keys: Test = toml::from_str(&strs).unwrap();

    assert_eq!(strs, right);
    assert_eq!(key, keys);

    let key = Test {
        keys: Keys(vec![Key {
            code: KeyCode::Char(' '),
            ..Default::default()
        }]),
    };
    let strs = toml::to_string(&key).unwrap();

    let right = "keys = \"<Space>\"\n";
    let keys: Test = toml::from_str(&strs).unwrap();

    assert_eq!(strs, right);
    assert_eq!(key, keys);
}

#[test]
fn serde_keymap() {
    let key = Test {
        keys: Keys(vec![Key {
            ctrl: true,
            shift: true,
            alt: true,
            code: KeyCode::BackTab,
        }]),
    };
    let strs = toml::to_string(&key).unwrap();

    let right = "keys = \"<C-A-S-Tab>\"\n";
    let keys: Test = toml::from_str(&strs).unwrap();

    assert_eq!(strs, right);
    assert_eq!(key, keys);

    let key = Test {
        keys: Keys(vec![Key {
            ctrl: true,
            shift: false,
            alt: true,
            code: KeyCode::Char('a'),
        }]),
    };
    let strs = toml::to_string(&key).unwrap();

    let right = "keys = \"<C-A-a>\"\n";
    let keys: Test = toml::from_str(&strs).unwrap();

    assert_eq!(strs, right);
    assert_eq!(key, keys);
}

#[test]
fn serde_keymap_long() {
    let test = r#"keys = "<C-A-S-Tab>abc""#;
    let pat: Test = toml::from_str(test).unwrap();
    let res = Keys(vec![
        Key {
            ctrl: true,
            shift: true,
            alt: true,
            code: KeyCode::BackTab,
        },
        Key {
            code: KeyCode::Char('a'),
            ..Default::default()
        },
        Key {
            code: KeyCode::Char('b'),
            ..Default::default()
        },
        Key {
            code: KeyCode::Char('c'),
            ..Default::default()
        },
    ]);
    assert_eq!(pat, Test { keys: res });

    let test = r#"keys = "<C-A-S-Tab>abcABC<S-s>""#;
    let pat: Test = toml::from_str(test).unwrap();
    let res = Keys(vec![
        Key {
            ctrl: true,
            shift: true,
            alt: true,
            code: KeyCode::BackTab,
        },
        Key {
            code: KeyCode::Char('a'),
            ..Default::default()
        },
        Key {
            code: KeyCode::Char('b'),
            ..Default::default()
        },
        Key {
            code: KeyCode::Char('c'),
            ..Default::default()
        },
        Key {
            shift: true,
            code: KeyCode::Char('A'),
            ..Default::default()
        },
        Key {
            shift: true,
            code: KeyCode::Char('B'),
            ..Default::default()
        },
        Key {
            shift: true,
            code: KeyCode::Char('C'),
            ..Default::default()
        },
        Key {
            shift: true,
            code: KeyCode::Char('S'),
            ..Default::default()
        },
    ]);
    assert_eq!(pat, Test { keys: res });
}

#[test]
fn serde_keymap_ignore_case() {
    // ignore case on `<S-*>`
    let test = r#"keys = "<S-a><S-A>A""#;
    let pat: Test = toml::from_str(test).unwrap();
    let res = Keys(vec![
        Key {
            shift: true,
            code: KeyCode::Char('A'),
            ..Default::default()
        },
        Key {
            shift: true,
            code: KeyCode::Char('A'),
            ..Default::default()
        },
        Key {
            shift: true,
            code: KeyCode::Char('A'),
            ..Default::default()
        },
    ]);
    assert_eq!(pat, Test { keys: res });
}
