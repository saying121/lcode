use std::{
    fs::{create_dir_all, write, OpenOptions},
    io::Read,
};

use crate::config::user_nest::Urls;

use super::{global::*, User};
use miette::{Error, IntoDiagnostic};
use tracing::{instrument, trace, warn};

/// generate default config
///
/// * `force`: when true will override your config
/// * `tongue`:  "cn"  "en"
pub fn gen_default_conf(tongue: &str) -> Result<(), Error> {
    let user = User::new(tongue);
    let config_path = glob_config_path();
    create_dir_all(
        config_path
            .parent()
            .unwrap_or_else(|| glob_config_path()),
    )
    .into_diagnostic()?;

    if !config_path.exists() {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(config_path)
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
    let config_path = glob_config_path();
    if !config_path.exists() {
        gen_default_conf("")?;
    }
    let mut cf = OpenOptions::new()
        .read(true)
        .open(config_path)
        .into_diagnostic()?;

    let mut content = String::new();
    cf.read_to_string(&mut content)
        .into_diagnostic()?;

    let mut user: User = toml::from_str(&content).into_diagnostic()?;
    user.urls = Urls::new(&user.url_suffix);

    if user.code_dir.starts_with("~") {
        let mut path = user
            .code_dir
            .to_string_lossy()
            .to_string();
        let path = path.split_off(2);
        let home = dirs::home_dir().unwrap();
        let mut code_dir = home;
        code_dir.push(path);
        user.code_dir = code_dir;
    }

    trace!("the get user config: {:#?}", user);

    Ok(user)
}
