pub mod chromium;
pub mod firefox;

use std::fmt::Display;

use miette::Result;
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum Browser {
    Edge      = 0,
    Chrome    = 1,
    #[default]
    Firefox   = 2,
    Librewolf = 3
}

impl<'de> Deserialize<'de> for Browser {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        struct VariantString;

        impl<'d> Visitor<'d> for VariantString {
            type Value = Browser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("failed Deserialize `Browser`")
            }
            fn visit_str<E>(self, v: &str) -> std::prelude::v1::Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                let res = match v {
                    "edge" => Browser::Edge,
                    "firefox" => Browser::Firefox,
                    "chrome" => Browser::Chrome,
                    "librewolf" => Browser::Librewolf,
                    _ => Browser::Firefox
                };
                Ok(res)
            }
            fn visit_string<E>(self, v: String) -> std::prelude::v1::Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                let res = match v.as_str() {
                    "edge" => Browser::Edge,
                    "firefox" => Browser::Firefox,
                    "chrome" => Browser::Chrome,
                    "librewolf" => Browser::Librewolf,
                    _ => Browser::Firefox
                };
                Ok(res)
            }
        }
        deserializer.deserialize_string(VariantString)
    }
}

impl Serialize for Browser {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        match *self {
            Self::Edge => serializer.serialize_unit_variant("Browser", 0, "edge"),
            Self::Chrome => serializer.serialize_unit_variant("Browser", 1, "chrome"),
            Self::Firefox => serializer.serialize_unit_variant("Browser", 2, "firefox"),
            Self::Librewolf => serializer.serialize_unit_variant("Browser", 3, "librewolf")
        }
    }
}

impl From<&str> for Browser {
    fn from(value: &str) -> Self {
        match value {
            "edge" => Self::Edge,
            "chrome" => Self::Chrome,
            "firefox" => Self::Firefox,
            "librewolf" => Self::Librewolf,
            _ => Self::Firefox
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cookies {
    pub csrf:    String,
    pub session: String
}

impl Cookies {
    pub fn is_completion(&self) -> bool {
        !(self.csrf.is_empty() || self.session.is_empty())
    }
}

impl Display for Cookies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("LEETCODE_SESSION={};csrftoken={};", self.session, self.csrf).fmt(f)
    }
}
/// get csrf and session
///
/// * `borwser`: firefox, librewolf, edge, chrome
pub async fn get_cookie<T>(borwser: T, host: &str) -> Result<Cookies>
where
    T: Into<Browser>
{
    let res = match borwser.into() {
        Browser::Firefox => firefox::get_session_csrf(Browser::Firefox, host).await?,
        Browser::Librewolf => firefox::get_session_csrf(Browser::Librewolf, host).await?,
        Browser::Edge => chromium::get_session_csrf(Browser::Edge, host).await?,
        Browser::Chrome => chromium::get_session_csrf(Browser::Chrome, host).await?
    };

    Ok(res)
}
