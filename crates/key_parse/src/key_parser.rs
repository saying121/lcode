#![allow(clippy::string_slice, reason = "todo: make it more grace")]

use crossterm::event::KeyCode;
use miette::bail;
use serde::{Deserialize, Deserializer, Serializer};

use crate::keymap::Key;

/// str to keycode
fn match_key(i: &str) -> miette::Result<KeyCode> {
    let res = match i {
        "Space" | "space" => KeyCode::Char(' '),
        "Bs" | "bs" => KeyCode::Backspace,
        "Cr" | "cr" => KeyCode::Enter,
        "Left" | "left" => KeyCode::Left,
        "Right" | "right" => KeyCode::Right,
        "Up" | "up" => KeyCode::Up,
        "Down" | "down" => KeyCode::Down,
        "Home" | "home" => KeyCode::Home,
        "End" | "end" => KeyCode::End,
        "PageUp" | "pageup" => KeyCode::PageUp,
        "PageDown" | "pagedown" => KeyCode::PageDown,
        "Tab" | "tab" => KeyCode::Tab,
        // "BackTab" | "backtab" =>  KeyCode::BackTab,
        "Del" | "del" => KeyCode::Delete,
        "Insert" | "insert" => KeyCode::Insert,
        "F1" | "f1" => KeyCode::F(1),
        "F2" | "f2" => KeyCode::F(2),
        "F3" | "f3" => KeyCode::F(3),
        "F4" | "f4" => KeyCode::F(4),
        "F5" | "f5" => KeyCode::F(5),
        "F6" | "f6" => KeyCode::F(6),
        "F7" | "f7" => KeyCode::F(7),
        "F8" | "f8" => KeyCode::F(8),
        "F9" | "f9" => KeyCode::F(9),
        "F10" | "f10" => KeyCode::F(10),
        "F11" | "f11" => KeyCode::F(11),
        "F12" | "f12" => KeyCode::F(12),
        "Esc" | "esc" => KeyCode::Esc,
        not_support => bail!("not support key: {not_support}"),
    };
    Ok(res)
}
/// keycode to str
fn match_keycode(code: KeyCode) -> miette::Result<&'static str> {
    let temp = match code {
        KeyCode::Char(' ') => "<Space>",
        KeyCode::Char(ch) => ch.to_string().leak(),

        KeyCode::Backspace => "<Bs>",
        KeyCode::Enter => "<Cr>",
        KeyCode::Left => "<Left>",
        KeyCode::Right => "<Right>",
        KeyCode::Up => "<Up>",
        KeyCode::Down => "<Down>",
        KeyCode::Home => "<Home>",
        KeyCode::End => "<End>",

        KeyCode::PageUp => "<PageUp>",
        KeyCode::PageDown => "<PageDown>",
        KeyCode::Tab => "<Tab>",
        KeyCode::BackTab => "<S-Tab>",
        KeyCode::Delete => "<Del>",
        KeyCode::Insert => "<Insert>",

        KeyCode::F(1) => "<F1>",
        KeyCode::F(2) => "<F2>",
        KeyCode::F(3) => "<F3>",
        KeyCode::F(4) => "<F4>",
        KeyCode::F(5) => "<F5>",
        KeyCode::F(6) => "<F6>",
        KeyCode::F(7) => "<F7>",
        KeyCode::F(8) => "<F8>",
        KeyCode::F(9) => "<F9>",
        KeyCode::F(10) => "<F10>",
        KeyCode::F(11) => "<F11>",
        KeyCode::F(12) => "<F12>",

        KeyCode::Esc => "<Esc>",
        // KeyCode::CapsLock => todo!(),
        // KeyCode::ScrollLock => todo!(),
        // KeyCode::NumLock => todo!(),
        // KeyCode::PrintScreen => todo!(),
        // KeyCode::Pause => todo!(),
        // KeyCode::Menu => todo!(),
        // KeyCode::KeypadBegin => todo!(),
        // KeyCode::Media(_) => todo!(),
        // KeyCode::Modifier(_) => todo!(),
        // KeyCode::Null => todo!(),
        not_support => bail!("not support key: {not_support:?}"),
    };
    Ok(temp)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Key>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Err(serde::de::Error::custom("no keys"));
    }
    let len = s.len();
    let mut res = vec![];
    let (mut left, mut right) = (0, 1);
    let mut cur = &s[left..right];
    loop {
        let mut key = Key::default();

        // be like <S-C-abc...>
        if cur.starts_with('<') {
            left += 1;
            let Some(index) = s[left..].find('>')
            else {
                return Err(serde::de::Error::custom(
                    "Not complete, found `<` but can't found `>`",
                ));
            };
            right = index + left;

            let mut pat = s[left..right]
                .split_inclusive('-')
                .peekable();

            while let Some(i) = pat.next() {
                match i {
                    "S-" | "s-" => key.shift = true,
                    "A-" | "a-" | "M-" | "m-" => key.alt = true,
                    "C-" | "c-" => key.ctrl = true,

                    end if pat.peek().is_none() => {
                        if let Ok(keycode) = match_key(end) {
                            key.code = keycode;
                        }
                        else {
                            let last = end
                                .chars()
                                .next()
                                .expect("key_parse failed");
                            // To convert characters without `shift` to lowercase
                            if !key.shift && last.is_ascii_uppercase() {
                                let last = last.to_ascii_lowercase();
                                key.code = KeyCode::Char(last);
                            }
                            // To convert characters with `shift` to uppercase
                            else if key.shift && last.is_ascii_lowercase() {
                                let last = last.to_ascii_uppercase();
                                key.code = KeyCode::Char(last);
                            }
                            else {
                                key.code = KeyCode::Char(last);
                            }
                        }
                    },
                    unknown => {
                        return Err(serde::de::Error::custom(format!(
                            "not support key :{unknown}"
                        )));
                    },
                }
            }
            // convert `S-Tab` to `BackTab`
            if key.shift && key.code == KeyCode::Tab {
                key.code = KeyCode::BackTab;
            }

            left = right + 1;
            right = left + 1;
        }
        else {
            let code = cur
                .chars()
                .next()
                .expect("key_parse failed");
            key.code = KeyCode::Char(code);
            if code.is_ascii_uppercase() {
                key.shift = true;
            }

            left = right;
            right += 1;
        }

        res.push(key);
        if left >= len {
            break;
        }
        cur = &s[left..right];
    }

    Ok(res)
}
pub fn serialize<S>(v: &Vec<Key>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut res = String::with_capacity(16);
    for key in v {
        if key.shift || key.ctrl || key.alt {
            res.push('<');
            let shift = if key.code == KeyCode::BackTab {
                false
            }
            else {
                key.shift
            };

            let re = match (key.ctrl, shift, key.alt) {
                (true, true, true) => "C-S-A-",
                (true, false, true) => "C-A-",
                (true, true, false) => "C-S-",
                (true, false, false) => "C-",
                (false, true, true) => "S-A-",
                (false, true, false) => "S-",
                (false, false, true) => "A-",
                (false, false, false) => "",
            };
            if !re.is_empty() {
                res.push_str(re);
            }
            let Ok(temp) = match_keycode(key.code)
            else {
                return Err(serde::ser::Error::custom("not support key"));
            };
            res.push_str(temp.trim_matches(|v| v == '<' || v == '>'));
            res.push('>');
        }
        else {
            let Ok(temp) = match_keycode(key.code)
            else {
                return Err(serde::ser::Error::custom("not support key"));
            };
            res.push_str(temp);
        }
    }
    serializer.serialize_str(&res)
}
