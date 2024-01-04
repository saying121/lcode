use std::path::PathBuf;

use crate::Browser;

#[cfg(target_os = "linux")]
pub fn linux_path(browser: Browser) -> PathBuf {
    const EDGE_LINUX: &str = "microsoft-edge/Default/Cookies";
    const CHROME_LINUX1: &str = "google-chrome/Profile 1/Cookies";
    const CHROME_LINUX: &str = "google-chrome/Default/Cookies";

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

#[cfg(target_os = "macos")]
pub(crate) fn macos_path(browser: Browser) -> PathBuf {
    const EDGE_MAC: &str = "Microsoft Edge/Default/Cookies";
    const CHROME_MAC: &str = "Google/Chrome/Default/Cookies";

    let mut cookie_dir = dirs::config_dir().expect("get config dir failed");
    let v = match browser {
        Browser::Edge => EDGE_MAC,
        Browser::Chrome => CHROME_MAC,
        _ => EDGE_MAC,
    };
    cookie_dir.push(v);
    cookie_dir
}

#[cfg(target_os = "windows")]
pub(crate) fn win_path(browser: Browser) -> PathBuf {
    const EDGE_WIN: &str = r#"Microsoft\Edge\User Data\Default\Cookies"#;
    const CHROME_WIN: &str = r#"Google\Chrome\User Data\Default\Cookies"#;

    let mut cookie_dir = dirs::cache_dir().expect("get config dir failed");
    let v = match browser {
        Browser::Edge => EDGE_WIN,
        Browser::Chrome => CHROME_WIN,
        _ => EDGE_WIN,
    };
    cookie_dir.push(v);
    cookie_dir
}

pub fn get_browser_cookies_path(browser: Browser) -> PathBuf {
    #[cfg(target_os = "linux")]
    let cookie_dir = linux_path(browser);
    #[cfg(target_os = "macos")]
    let cookie_dir = macos_path(browser);
    #[cfg(target_os = "windows")]
    let cookie_dir = win_path(browser);

    cookie_dir
}
