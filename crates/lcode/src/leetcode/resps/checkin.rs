use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct CheckInData {
    pub data: CheckIn,
}
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct CheckIn {
    #[serde(default)]
    pub checkin: CheckedIn,
}
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct CheckedIn {
    #[serde(default, alias = "checkedIn")]
    pub checked_in: bool,
    #[serde(default)]
    pub error:      Option<()>,
    #[serde(default)]
    pub ok:         bool,
}
