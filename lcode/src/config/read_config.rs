use std::fs::{self, write, OpenOptions};

use miette::{IntoDiagnostic, Result};
use tracing::{instrument, warn};

use super::{global::*, User};
use crate::config::{user_nest::Urls, Config};

#[derive(Copy, Clone)]
pub enum Tongue {
    Cn,
    En,
}

/// generate default config
///
/// * `force`: when true will override your config
/// * `tongue`:  "cn"  "en"
pub fn gen_default_conf(tongue: Tongue) -> Result<()> {
    let user = User::new(tongue);
    /// the `$ident` need `global_$ident_path` and `user.$ident`
    macro_rules! the_configs {
        ($($conf:ident), *) => {
            paste::paste!{
                $(
                    if ![<glob_ $conf _path>]().exists() {
                        OpenOptions::new()
                            .create(true)
                            .write(true)
                            .open([<glob_ $conf _path>]())
                            .into_diagnostic()?;
                        let toml = toml::to_string(&user.$conf).into_diagnostic()?;
                        write([<glob_ $conf _path>](), toml).into_diagnostic()?;
                    }
                )*
            }
        };
    }
    the_configs!(config, cookies, langs);

    Ok(())
}

/// get the user's config
/// please first use `global_user_config()` for get config
#[instrument]
pub fn get_user_conf() -> Result<User> {
    if !(glob_config_path().exists() && glob_cookies_path().exists() && glob_langs_path().exists())
    {
        gen_default_conf(Tongue::En)?;
    }

    let config = fs::read_to_string(glob_config_path()).into_diagnostic()?;
    let mut config: Config = toml::from_str(&config)
        .into_diagnostic()
        .expect(
            "missing some field, you can backup of `config.toml` as `config.toml.bak` for auto \
             generate",
        );
    let urls = Urls::new(&config.url_suffix);

    if config.code_dir.starts_with("~") {
        let mut path = config
            .code_dir
            .to_string_lossy()
            .to_string();
        let path = path.split_off(2);
        let mut code_dir = dirs::home_dir().unwrap();
        code_dir.push(path);
        config.code_dir = code_dir;
    }
    let langs = fs::read_to_string(glob_langs_path())
        .into_diagnostic()
        .unwrap();
    let langs = toml::from_str(&langs)
        .into_diagnostic()
        .expect(
            "missing some field, you can backup of `langs.toml` as `langs.toml.bak` for auto \
             generate",
        );

    let cookies = fs::read_to_string(glob_cookies_path())
        .into_diagnostic()
        .unwrap();
    let cookies = toml::from_str(&cookies)
        .into_diagnostic()
        .expect(
            "missing some field, you can backup of `cookies.toml` as `cookies.toml.bak` for auto \
             generate",
        );

    let user = User {
        urls,
        config,
        cookies,
        langs,
    };

    Ok(user)
}
