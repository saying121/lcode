use std::path::PathBuf;

use crate::Browser;

#[cfg(target_os = "linux")]
pub(crate) fn linux_path(browser: Browser) -> (PathBuf, &'static str) {
    const FIREFOX_LINUX: &str = ".mozilla/firefox";
    const LIBREWOLF_LINUX: &str = ".librewolf";

    let mut home = dirs::home_dir().expect("get home dir failed");
    let temp = match browser {
        Browser::Firefox => FIREFOX_LINUX,
        Browser::Librewolf => LIBREWOLF_LINUX,
        _ => FIREFOX_LINUX,
    };
    home.push(format!("{}/profiles.ini", temp));
    (home, temp)
}

#[cfg(target_os = "macos")]
pub(crate) fn macos_path(browser: Browser) -> (PathBuf, &'static str) {
    const FIREFOX_MAC: &str = "Library/Application Support/Firefox";
    const LIBREWOLF_MAC: &str = "Library/Application Support/librewolf";

    let mut home = dirs::home_dir().expect("get home dir failed");
    let temp = match browser {
        Browser::Firefox => FIREFOX_MAC,
        Browser::Librewolf => LIBREWOLF_MAC,
        _ => FIREFOX_MAC,
    };
    home.push(format!("{}/profiles.ini", temp));
    (home, temp)
}

#[cfg(target_os = "windows")]
pub(crate) fn win_path(browser: Browser) -> (PathBuf, &'static str) {
    const FIREFOX_WIN: &str = r"Mozilla\Firefox";
    const LIBREWOLF_WIN: &str = "librewolf";

    let mut home = dirs::home_dir().expect("get home dir failed");
    let temp = match browser {
        Browser::Firefox => FIREFOX_WIN,
        Browser::Librewolf => LIBREWOLF_WIN,
        _ => FIREFOX_WIN,
    };
    home.push(format!("{}/profiles.ini", temp));
    (home, temp)
}
