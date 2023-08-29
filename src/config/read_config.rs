use std::{
    collections::VecDeque,
    fs::{create_dir_all, write, OpenOptions},
    io::Read,
};

use super::{global::*, user_nest::*, User};
use miette::{Error, IntoDiagnostic};
use tracing::{instrument, trace, warn};

/// generate default config
///
/// * `force`: when true will override your config
/// * `tongue`:  "cn"  "en"
pub fn gen_default_conf(tongue: &str) -> Result<(), Error> {
    let user = User::new(tongue);
    let config_path = init_config_path();
    create_dir_all(
        config_path
            .parent()
            .unwrap_or_else(|| init_config_path()),
    )
    .into_diagnostic()?;

    if !config_path.exists() {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&config_path)
            .into_diagnostic()?;
        let config_toml = toml::to_string(&user).into_diagnostic()?;
        write(config_path, config_toml).into_diagnostic()?;
    }

    Ok(())
}

/// get the user's config
/// please use global_user_config() for get config
#[instrument]
pub(in crate::config) fn get_user_conf() -> Result<User, Error> {
    let config_path = init_config_path();
    if !config_path.exists() {
        gen_default_conf("")?;
    }
    let mut cf = OpenOptions::new()
        .read(true)
        .open(&config_path)
        .into_diagnostic()?;

    let mut content = String::new();
    cf.read_to_string(&mut content)
        .into_diagnostic()?;
    let cf_str: toml::Value = toml::from_str(&content).into_diagnostic()?;
    trace!("user config toml Value: {:#?}", cf_str);

    let user: User = User {
        num_sublist: cf_str
            .get("num_sublist")
            .and_then(|v| v.as_integer())
            .map(|v| v as u32)
            .unwrap_or_default(),
        page_size: cf_str
            .get("page_size")
            .and_then(|v| v.as_integer())
            .map(|v| v as usize)
            .unwrap_or_default(),
        column: cf_str
            .get("column")
            .and_then(|v| v.as_integer())
            .map(|v| v as usize)
            .unwrap_or_else(|| {
                warn!("user config parser column error, use 4");
                4
            }),
        translate: cf_str
            .get("translate")
            .and_then(|v| v.as_bool())
            .unwrap_or_else(|| {
                warn!("user config parser column error, use default");
                false
            }),
        cookies: cf_str
            .get("cookies")
            .and_then(|v| v.as_table())
            .map_or_else(
                || {
                    warn!("user config parser cookies error, use default");
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
                    warn!("user config parser editor error, use default");
                    VecDeque::from([get_editor().clone()])
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
        url_suffix: cf_str
            .get("url_suffix")
            .map_or_else(
                || {
                    warn!("user config parser urls error, use default");
                    Urls::default()
                },
                |v| Urls::new(v.as_str().unwrap_or("com")),
            ),
        ..Default::default()
    };

    trace!("the get user config: {:#?}", user);

    Ok(user)
}
