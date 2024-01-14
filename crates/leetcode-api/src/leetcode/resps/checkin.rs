use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckInData {
    pub data: CheckIn,
}
#[derive(Default, Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckIn {
    #[serde(default)]
    pub checkin: CheckedIn,
}
#[derive(Default, Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckedIn {
    #[serde(default, alias = "checkedIn")]
    pub checked_in: bool,
    #[serde(default)]
    pub error:      Option<()>,
    #[serde(default)]
    pub ok:         bool,
}

#[derive(Default,Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TotalPoints {
    #[serde(default)]
    points: u64,
}
