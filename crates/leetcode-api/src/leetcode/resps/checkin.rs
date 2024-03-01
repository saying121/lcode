use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct CheckInData {
    data: CheckIn,
}

impl CheckInData {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub const fn checkin_ok(&self) -> bool {
        self.data.checkin.ok
    }
}
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
struct CheckIn {
    #[serde(default)]
    checkin: CheckedIn,
}
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
struct CheckedIn {
    #[serde(default, alias = "checkedIn")]
    checked_in: bool,
    #[serde(default)]
    error:      Option<()>,
    #[serde(default)]
    ok:         bool,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct TotalPoints {
    #[serde(default)]
    points: u64,
}

impl TotalPoints {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub const fn points(&self) -> u64 {
        self.points
    }
    pub fn add_point(&mut self, num: u64) {
        self.points += num;
    }
}
