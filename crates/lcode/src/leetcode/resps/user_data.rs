use ratatui::prelude::*;
use serde::{Deserialize, Serialize};

use crate::render::Render;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct GlobData {
    #[serde(default)]
    pub data: UserStatusData,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct UserStatusData {
    #[serde(default, alias = "userStatus")]
    pub user_status: UserStatus,
}
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct UserStatus {
    #[serde(default, alias = "activeSessionId")]
    pub active_session_id: u32,
    #[serde(default)]
    pub avatar:            Option<String>,
    #[serde(default, alias = "checkedInToday")]
    pub checked_in_today:  bool,
    #[serde(default, alias = "isAdmin")]
    pub is_admin:          bool,
    #[serde(default, alias = "isPremium")]
    pub is_premium:        Option<bool>,
    #[serde(default, alias = "isSignedIn")]
    pub is_signed_in:      bool,
    #[serde(default, alias = "isSuperuser")]
    pub is_superuser:      bool,
    #[serde(default, alias = "isTranslator")]
    pub is_translator:     bool,
    #[serde(default, alias = "isVerified")]
    pub is_verified:       bool,
    #[serde(default)]
    pub permissions:       Vec<String>,
    #[serde(default, alias = "realName")]
    pub real_name:         Option<String>,
    #[serde(default, alias = "userSlug")]
    pub user_slug:         Option<String>,
    #[serde(default)]
    pub username:          String,
}

impl Render for UserStatus {
    fn to_tui_vec(&self) -> Vec<Line> {
        vec![
            format!("User: {}", self.username).into(),
            format!("Is Verified: {}", self.is_verified).into(),
            format!("Is Premium: {}", self.is_premium.unwrap_or_default()).into(),
            format!("Is admin: {}", self.is_admin).into(),
            format!("Checked In Today: {}", self.checked_in_today).into(),
            format!("Is Superuser: {}", self.is_superuser).into(),
            format!("Is Translator: {}", self.is_translator).into(),
        ]
    }
}
