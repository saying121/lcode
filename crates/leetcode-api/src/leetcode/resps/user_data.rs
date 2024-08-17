use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub struct GlobData {
    #[serde(default)]
    data: UserStatusData,
}

impl GlobData {
    pub fn user_status(self) -> UserStatus {
        self.data.user_status
    }
}

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
struct UserStatusData {
    #[serde(default, alias = "userStatus")]
    user_status: UserStatus,
}

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub struct UserStatus {
    #[serde(default, alias = "activeSessionId")]
    pub active_session_id: u32,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default, alias = "checkedInToday")]
    pub checked_in_today: bool,
    #[serde(default, alias = "isAdmin")]
    pub is_admin: bool,
    #[serde(default, alias = "isPremium")]
    pub is_premium: Option<bool>,
    #[serde(default, alias = "isSignedIn")]
    pub is_signed_in: bool,
    #[serde(default, alias = "isSuperuser")]
    pub is_superuser: bool,
    #[serde(default, alias = "isTranslator")]
    pub is_translator: bool,
    #[serde(default, alias = "isVerified")]
    pub is_verified: bool,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default, alias = "realName")]
    pub real_name: Option<String>,
    #[serde(default, alias = "userSlug")]
    pub user_slug: Option<String>,
    #[serde(default)]
    pub username: String,
}
