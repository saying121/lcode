use crossterm::event::KeyCode;
use key_parse::keymap::{Key, Keys};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Test {
    keys: Keys,
}

#[test]
fn serde_keymap() {
    let key = Test {
        keys: Keys(vec![Key {
            ctrl:  true,
            shift: true,
            alt:   true,
            code:  KeyCode::BackTab,
        }]),
    };
    let strs = toml::to_string(&key).unwrap();

    let right = "keys = \"<C-A-S-Tab>\"\n";
    pretty_assertions::assert_eq!(strs, right);

    let keys: Test = toml::from_str(&strs).unwrap();
    pretty_assertions::assert_eq!(key, keys);

    let test = r#"keys = "<C-A-S-Tab>abc""#;
    let pat: Test = toml::from_str(test).unwrap();
    let res = Keys(vec![
        Key {
            ctrl:  true,
            shift: true,
            alt:   true,
            code:  KeyCode::BackTab,
        },
        Key {
            ctrl:  false,
            shift: false,
            alt:   false,
            code:  KeyCode::Char('a'),
        },
        Key {
            ctrl:  false,
            shift: false,
            alt:   false,
            code:  KeyCode::Char('b'),
        },
        Key {
            ctrl:  false,
            shift: false,
            alt:   false,
            code:  KeyCode::Char('c'),
        },
    ]);
    pretty_assertions::assert_eq!(
        pat,
        Test {
            keys: res
        }
    );

    let test = r#"keys = "<C-A-S-Tab>abcABC<S-s>""#;
    let pat: Test = toml::from_str(test).unwrap();
    let res = Keys(vec![
        Key {
            ctrl:  true,
            shift: true,
            alt:   true,
            code:  KeyCode::BackTab,
        },
        Key {
            ctrl:  false,
            shift: false,
            alt:   false,
            code:  KeyCode::Char('a'),
        },
        Key {
            ctrl:  false,
            shift: false,
            alt:   false,
            code:  KeyCode::Char('b'),
        },
        Key {
            ctrl:  false,
            shift: false,
            alt:   false,
            code:  KeyCode::Char('c'),
        },
        Key {
            ctrl:  false,
            shift: false,
            alt:   false,
            code:  KeyCode::Char('A'),
        },
        Key {
            ctrl:  false,
            shift: false,
            alt:   false,
            code:  KeyCode::Char('B'),
        },
        Key {
            ctrl:  false,
            shift: false,
            alt:   false,
            code:  KeyCode::Char('C'),
        },
        Key {
            ctrl:  false,
            shift: true,
            alt:   false,
            code:  KeyCode::Char('s'),
        },
    ]);
    pretty_assertions::assert_eq!(
        pat,
        Test {
            keys: res
        }
    );

    // maybe need ignore case on `<S-*>`
    let test = r#"keys = "<S-a><S-A>""#;
    let pat: Test = toml::from_str(test).unwrap();
    let res = Keys(vec![
        Key {
            shift: true,
            code: KeyCode::Char('a'),
            ..Default::default()
        },
        Key {
            shift: true,
            code: KeyCode::Char('A'),
            ..Default::default()
        },
    ]);
    pretty_assertions::assert_eq!(
        pat,
        Test {
            keys: res
        }
    );
}
