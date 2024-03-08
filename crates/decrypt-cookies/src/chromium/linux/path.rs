use std::path::PathBuf;

use crate::Browser;

const EDGE_LINUX: &str = "microsoft-edge/Default/Cookies";
const CHROME_LINUX: &str = "google-chrome/Default/Cookies";
const CHROME_LINUX1: &str = "google-chrome/Profile 1/Cookies";

pub fn linux_path(browser: Browser) -> PathBuf {
    let mut cookie_dir = dirs::config_dir().expect("get config dir failed");
    let v = match browser {
        Browser::Edge => EDGE_LINUX,
        Browser::Chrome => CHROME_LINUX1,
        _ => EDGE_LINUX,
    };
    cookie_dir.push(v);

    if browser == Browser::Chrome && !cookie_dir.exists() {
        cookie_dir = dirs::config_dir().expect("get config dir failed");
        cookie_dir.push(CHROME_LINUX);
    }
    cookie_dir
}
