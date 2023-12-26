use std::ops::{Deref, DerefMut};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

pub const CTRL: u8 = 0b001;
pub const SHIFT: u8 = 0b010;
pub const ALT: u8 = 0b100;
pub const NO_CONTROL: u8 = 0b000;

#[derive(Debug, PartialEq, Eq)]
pub struct Key {
    pub ctrl:  bool,
    pub shift: bool,
    pub alt:   bool,
    pub code:  KeyCode,
}

impl Key {
    /// # Example
    ///
    /// ```rust
    /// use crossterm::event::KeyCode;
    /// use key_parse::keymap::*;
    ///
    /// let key = Key::new(CTRL | ALT, KeyCode::Tab);
    /// let res = Key {
    ///     ctrl:  true,
    ///     shift: false,
    ///     alt:   true,
    ///     code:  KeyCode::Tab,
    /// };
    /// assert_eq!(key, res);
    /// ```
    pub const fn new(control: u8, code: KeyCode) -> Self {
        Self {
            ctrl: control & CTRL != 0,
            shift: control & SHIFT != 0,
            alt: control & ALT != 0,
            code,
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self {
            code:  KeyCode::Null,
            shift: false,
            ctrl:  false,
            alt:   false,
        }
    }
}

impl From<KeyEvent> for Key {
    fn from(value: KeyEvent) -> Self {
        let temp = value
            .modifiers
            .contains(KeyModifiers::SHIFT);

        let shift = match value.code {
            KeyCode::Char(ch) => ch.is_ascii_uppercase() || temp,
            _ => temp,
        };

        Self {
            ctrl: value
                .modifiers
                .contains(KeyModifiers::CONTROL),
            shift,
            alt: value
                .modifiers
                .contains(KeyModifiers::ALT),
            code: value.code,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Keys(#[serde(default, with = "super::key_parser")] pub Vec<Key>);

impl Deref for Keys {
    type Target = Vec<Key>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Keys {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
