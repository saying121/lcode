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
    Librewolf = 3,
}

impl From<&str> for Browser {
    fn from(value: &str) -> Self {
        match value {
            "edge" => Self::Edge,
            "chrome" => Self::Chrome,
            "firefox" => Self::Firefox,
            "librewolf" => Self::Librewolf,
            _ => Self::Firefox,
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cookies {
    pub csrf:    String,
    pub session: String,
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
    T: Into<Browser>,
{
    let res = match borwser.into() {
        Browser::Firefox => firefox::get_session_csrf(Browser::Firefox, host).await?,
        Browser::Librewolf => firefox::get_session_csrf(Browser::Librewolf, host).await?,
        Browser::Edge => chromium::get_session_csrf(Browser::Edge, host).await?,
        Browser::Chrome => chromium::get_session_csrf(Browser::Chrome, host).await?,
    };

    Ok(res)
}
