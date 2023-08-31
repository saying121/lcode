pub mod chromium;
pub mod firefox;

use miette::Result;

use crate::config::user_nest::Cookies;

/// get csrf and session
///
/// * `borwser`: firefox, librewolf, edge, chrome
pub async fn get_cookie(borwser: &str) -> Result<Cookies> {
    let res = match borwser {
        "firefox" => firefox::get_session_csrf("firefox").await?,
        "librewolf" => firefox::get_session_csrf("librewolf").await?,
        "edge" => chromium::get_session_csrf("edge").await?,
        "chrome" => chromium::get_session_csrf("chrome").await?,
        _ => {
            let mut res = chromium::get_session_csrf("firefox").await?;
            if res.csrf.len() == 0 || res.session.len() == 0 {
                res = firefox::get_session_csrf("edge").await?;
            }
            if res.csrf.len() == 0 || res.session.len() == 0 {
                res = chromium::get_session_csrf("chrome").await?;
            }
            if res.csrf.len() == 0 || res.session.len() == 0 {
                res = firefox::get_session_csrf("librewolf").await?;
            }
            res
        }
    };

    Ok(res)
}
