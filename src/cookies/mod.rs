pub mod chromium_base;
mod chromium_base_entities;
pub mod ff_base;
mod ff_base_entities;

use miette::Result;

use self::{chromium_base::get_chrom_session_csrf, ff_base::get_ff_session_csrf};
use crate::config::user_nest::Cookies;

/// get csrf and session
///
/// * `borwser`: firefox, librewolf, edge
pub async fn get_cookie(borwser: &str) -> Result<Cookies> {
    let res = match borwser {
        "firefox" => get_ff_session_csrf("firefox").await?,
        "librewolf" => get_ff_session_csrf("librewolf").await?,
        "edge" => get_chrom_session_csrf().await?,
        _ => {
            let mut res = get_chrom_session_csrf().await?;
            if res.csrf.len() == 0 || res.session.len() == 0 {
                res = get_ff_session_csrf("firefox").await?;
            }
            if res.csrf.len() == 0 || res.session.len() == 0 {
                res = get_ff_session_csrf("librewolf").await?;
            }
            res
        }
    };

    Ok(res)
}
