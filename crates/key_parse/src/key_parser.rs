#![allow(clippy::string_slice)]

use crossterm::event::KeyCode;
use miette::bail;
use serde::{Deserialize, Deserializer, Serializer};

use crate::keymap::Key;

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

        if cur.starts_with('<') {
            left += 1;
            let Some(index) = s[left..].find('>')
            else {
                return Err(serde::de::Error::custom(
                    "not complete,find `<` but can't find `>`",
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

                    "Space" | "space" => key.code = KeyCode::Char(' '),
                    "Bs" | "bs" => key.code = KeyCode::Backspace,
                    "Cr" | "cr" => key.code = KeyCode::Enter,
                    "Left" | "left" => key.code = KeyCode::Left,
                    "Right" | "right" => key.code = KeyCode::Right,
                    "Up" | "up" => key.code = KeyCode::Up,
                    "Down" | "down" => key.code = KeyCode::Down,
                    "Home" | "home" => key.code = KeyCode::Home,
                    "End" | "end" => key.code = KeyCode::End,
                    "PageUp" | "pageup" => key.code = KeyCode::PageUp,
                    "PageDown" | "pagedown" => key.code = KeyCode::PageDown,
                    "Tab" | "tab" => key.code = KeyCode::Tab,
                    // "BackTab" | "backtab" => key.code = KeyCode::BackTab,
                    "Del" | "del" => key.code = KeyCode::Delete,
                    "Insert" | "insert" => key.code = KeyCode::Insert,
                    "F1" | "f1" => key.code = KeyCode::F(1),
                    "F2" | "f2" => key.code = KeyCode::F(2),
                    "F3" | "f3" => key.code = KeyCode::F(3),
                    "F4" | "f4" => key.code = KeyCode::F(4),
                    "F5" | "f5" => key.code = KeyCode::F(5),
                    "F6" | "f6" => key.code = KeyCode::F(6),
                    "F7" | "f7" => key.code = KeyCode::F(7),
                    "F8" | "f8" => key.code = KeyCode::F(8),
                    "F9" | "f9" => key.code = KeyCode::F(9),
                    "F10" | "f10" => key.code = KeyCode::F(10),
                    "F11" | "f11" => key.code = KeyCode::F(11),
                    "F12" | "f12" => key.code = KeyCode::F(12),
                    "Esc" | "esc" => key.code = KeyCode::Esc,
                    end if pat.peek().is_none() => {
                        if let Ok(keycode) = match_key(end) {
                            key.code = keycode;
                        }
                        else {
                            let last = end.chars().next().unwrap();
                            // To convert characters without shift to lowercase
                            if (!key.shift) && last.is_ascii_uppercase() {
                                let last = char::from_u32(last as u32 + 32).unwrap();
                                key.code = KeyCode::Char(last);
                            }
                            // To convert characters with shift to uppercase
                            else if key.shift && last.is_ascii_lowercase() {
                                let last = char::from_u32(last as u32 - 32).unwrap();
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
            // convert `"S-Tab"` to `BackTab`
            if key.shift && key.code == KeyCode::Tab {
                key.code = KeyCode::BackTab;
            }

            left = right + 1;
            right = left + 1;
        }
        else {
            let code = cur.chars().next().unwrap();
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
            let temp = match key.code {
                KeyCode::Char(' ') => "Space",
                KeyCode::Char(ch) => Box::leak(Box::new(ch.to_string())),

                KeyCode::Backspace => "Bs",
                KeyCode::Enter => "Cr",
                KeyCode::Left => "Left",
                KeyCode::Right => "Right",
                KeyCode::Up => "Up",
                KeyCode::Down => "Down",
                KeyCode::Home => "Home",
                KeyCode::End => "End",

                KeyCode::PageUp => "PageUp",
                KeyCode::PageDown => "PageDown",
                KeyCode::Tab => "Tab",
                KeyCode::BackTab => "S-Tab",
                KeyCode::Delete => "Del",
                KeyCode::Insert => "Insert",

                KeyCode::F(1) => "F1",
                KeyCode::F(2) => "F2",
                KeyCode::F(3) => "F3",
                KeyCode::F(4) => "F4",
                KeyCode::F(5) => "F5",
                KeyCode::F(6) => "F6",
                KeyCode::F(7) => "F7",
                KeyCode::F(8) => "F8",
                KeyCode::F(9) => "F9",
                KeyCode::F(10) => "F10",
                KeyCode::F(11) => "F11",
                KeyCode::F(12) => "F12",

                KeyCode::Esc => "Esc",
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
                _ => return Err(serde::ser::Error::custom("not support key")),
            };
            res.push_str(temp);
            res.push('>');
        }
        else {
            let temp = match key.code {
                KeyCode::Char(' ') => "<Space>",
                KeyCode::Char(ch) => Box::leak(Box::new(ch.to_string())),

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
                _ => return Err(serde::ser::Error::custom("not support key")),
            };
            res.push_str(temp);
        }
    }
    serializer.serialize_str(&res)
}
