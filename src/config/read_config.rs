use std::collections::VecDeque;

use crate::config::{init_code_dir, init_support_lang, user_nest::Cookies};

use super::{user_nest::Urls, User};
use miette::{Error, IntoDiagnostic};
use tokio::{
    fs::{create_dir_all, write, OpenOptions},
    io::AsyncReadExt,
};
use tracing::{instrument, trace, warn};

/// generate default config
///
/// * `force`: when true will override your config
/// * `tongue`: "Chinese" "cn" "English" "en"
pub async fn gen_default_conf(force: bool, tongue: &str) -> Result<(), Error> {
    let df_user = User::new(tongue);
    let config_dir = super::init_config_path();
    create_dir_all(config_dir.parent().unwrap())
        .await
        .into_diagnostic()?;

    if force || !config_dir.exists() {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&config_dir)
            .await
            .into_diagnostic()?;
        let config_toml = toml::to_string(&df_user).into_diagnostic()?;
        write(config_dir, config_toml)
            .await
            .into_diagnostic()?;
    }

    Ok(())
}

/// get the user's config
#[instrument]
pub async fn get_user_conf() -> Result<User, Error> {
    let config_path = super::init_config_path();
    if !config_path.exists() {
        gen_default_conf(false, "").await?;
    }
    let mut cf = OpenOptions::new()
        .read(true)
        .open(&config_path)
        .await
        .into_diagnostic()?;

    let mut content = String::new();
    cf.read_to_string(&mut content)
        .await
        .into_diagnostic()?;
    let cf_str: toml::Value = toml::from_str(&content).into_diagnostic()?;
    trace!("user config toml Value: {:#?}", cf_str);

    let user: User = User {
        cookies: cf_str
            .get("cookies")
            .and_then(|v| v.as_table())
            .map_or_else(
                || {
                    warn!("user config parser cookies error, use vim");
                    Cookies::default()
                },
                |v| Cookies {
                    csrf: v
                        .get("csrf")
                        .and_then(|v| v.as_str())
                        .unwrap_or_else(|| {
                            warn!("user config parser csrf error, use default");
                            ""
                        })
                        .to_string(),
                    session: v
                        .get("session")
                        .and_then(|v| v.as_str())
                        .unwrap_or_else(|| {
                            warn!("user config parser session error, use default");
                            ""
                        })
                        .to_string(),
                },
            ),
        editor: cf_str
            .get("editor")
            .and_then(|v| v.as_array())
            .map_or_else(
                || {
                    warn!("user config parser editor error, use vim");
                    VecDeque::from(["vim".to_string()])
                },
                |v| {
                    v.iter()
                        .map(|v| {
                            v.as_str()
                                .unwrap_or_default()
                                .to_string()
                        })
                        .collect()
                },
            ),
        lang: cf_str
            .get("lang")
            .map_or_else(
                || {
                    warn!("user config parser lang error, use rust");
                    "rust"
                },
                |v| {
                    let user_l = v.as_str().unwrap_or_default();
                    match init_support_lang().contains_key(user_l) {
                        true => user_l,
                        false => {
                            warn!("not support lang , use rust");
                            "rust"
                        }
                    }
                },
            )
            .to_string(),
        code_dir: cf_str
            .get("code_dir")
            .map_or_else(
                || {
                    warn!("user config parser code dir error, use default");
                    init_code_dir()
                        .to_string_lossy()
                        .to_string()
                },
                |v| {
                    v.as_str()
                        .unwrap_or_default()
                        .to_string()
                },
            )
            .into(),
        // urls: cf_str
        //     .get("urls")
        //     .and_then(|v| {
        //         Some(Urls {
        //             origin: v
        //                 .get("origin_url")
        //                 .map_or_else(||{
        //                     warn!("user config parser origin_url error, use default");
        //                     "https://leetcode.com"
        //                 }, |v| v.as_str().unwrap_or_default())
        //                 .to_string(),
        //             graphql: v
        //                 .get("graphql")
        //                 .map_or_else(||{
        //                     warn!("user config parser graphql error, use default");
        //                     "https://leetcode.com/graphql"
        //                 }, |v| v.as_str().unwrap_or_default())
        //                 .to_string(),
        //             all_problem_api: v
        //                 .get("all_problem_api")
        //                 .map_or_else(||{
        //                     warn!( "user config parser all_problem_api error, use default");
        //                     "https://leetcode.cn/api/problems/$category"
        //                 }, |v| v.as_str().unwrap_or_default())
        //                 .to_string(),
        //             submit: v
        //                 .get("submit")
        //                 .map_or_else(||{
        //                     warn!("user config parser submit error, use default");
        //                     "https://leetcode.cn/problems/$slug/submit/"
        //                 }, |v| v.as_str().unwrap_or_default())
        //                 .to_string(),
        //             test: v
        //                 .get("test")
        //                 .map_or_else(||{
        //                     warn!("user config parser test error, use default");
        //                     "https://leetcode.cn/problems/$slug/interpret_solution/"
        //                 }, |v| v.as_str().unwrap_or_default())
        //                 .to_string(),
        //             submissions: v
        //                 .get("submissions")
        //                 .map_or_else(||{
        //                     warn!("user config parser submissions error, use default");
        //                     "https://leetcode.cn/problems/$slug/interpret_solution/"
        //                 }, |v| v.as_str().unwrap_or_default())
        //                 .to_string(),
        //             favorites: v
        //                 .get("favorites")
        //                 .map_or_else(||{
        //                     warn!("user config parser favorites error, use default");
        //                     "https://leetcode.com/list/api/questions"
        //                 }, |v| v.as_str().unwrap_or_default())
        //                 .to_string(),
        //         })
        //     })
        //     .unwrap_or_default(),
        urls: cf_str.get("urls").map_or_else(
            || {
                warn!("user config parser urls error, use default");
                Urls::default()
            },
            |v| Urls {
                origin: v
                    .get("origin")
                    .map_or_else(
                        || {
                            warn!("user config parser origin_url error, use default");
                            "https://leetcode.com"
                        },
                        |v| v.as_str().unwrap_or_default(),
                    )
                    .to_string(),
                graphql: v
                    .get("graphql")
                    .map_or_else(
                        || {
                            warn!("user config parser graphql error, use default");
                            "https://leetcode.com/graphql"
                        },
                        |v| v.as_str().unwrap_or_default(),
                    )
                    .to_string(),
                all_problem_api: v
                    .get("all_problem_api")
                    .map_or_else(
                        || {
                            warn!(
                                "user config parser all_problem_api error, use default"
                            );
                            "https://leetcode.cn/api/problems/$category"
                        },
                        |v| v.as_str().unwrap_or_default(),
                    )
                    .to_string(),
                submit: v
                    .get("submit")
                    .map_or_else(
                        || {
                            warn!("user config parser submit error, use default");
                            "https://leetcode.cn/problems/$slug/submit/"
                        },
                        |v| v.as_str().unwrap_or_default(),
                    )
                    .to_string(),
                test: v
                    .get("test")
                    .map_or_else(
                        || {
                            warn!("user config parser test error, use default");
                            "https://leetcode.cn/problems/$slug/interpret_solution/"
                        },
                        |v| v.as_str().unwrap_or_default(),
                    )
                    .to_string(),
                submissions: v
                    .get("submissions")
                    .map_or_else(
                        || {
                            warn!("user config parser submissions error, use default");
                            "https://leetcode.cn/problems/$slug/interpret_solution/"
                        },
                        |v| v.as_str().unwrap_or_default(),
                    )
                    .to_string(),
                favorites: v
                    .get("favorites")
                    .map_or_else(
                        || {
                            warn!("user config parser favorites error, use default");
                            "https://leetcode.com/list/api/questions"
                        },
                        |v| v.as_str().unwrap_or_default(),
                    )
                    .to_string(),
            },
        ),
        ..Default::default()
    };

    trace!("the get user config: {:#?}", user);

    Ok(user)
}
