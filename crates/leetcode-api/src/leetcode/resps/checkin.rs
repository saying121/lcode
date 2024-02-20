use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct CheckInData {
    pub data: CheckIn,
}
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct CheckIn {
    #[serde(default)]
    pub checkin: CheckedIn,
}
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct CheckedIn {
    #[serde(default, alias = "checkedIn")]
    pub checked_in: bool,
    #[serde(default)]
    pub error:      Option<()>,
    #[serde(default)]
    pub ok:         bool,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct TotalPoints {
    #[serde(default)]
    pub points: u64,
}
