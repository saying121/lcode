use std::fs::{self, write, OpenOptions};

use miette::{IntoDiagnostic, Result};
use tracing::{instrument, warn};

use super::{global::*, user_nest::Suffix, User};
use crate::{
    config::{user_nest::Urls, Config},
    keymap::TuiKeyMap,
};

/// generate config
pub fn gen_config(suffix: Suffix) -> Result<()> {
    let user = User::new(suffix);

    /// the `$ident` need `global_$ident_path` and `user.$ident`
    macro_rules! the_configs {
        ($($conf:ident), *) => {
            paste::paste!{
                $(
                    if ![<G_ $conf:upper _PATH>].exists() {
                        OpenOptions::new()
                            .create(true)
                            .write(true)
                            .open(&*[<G_ $conf:upper _PATH>])
                            .into_diagnostic()?;
                        let toml = toml::to_string(&user.$conf).into_diagnostic()?;
                        write(&*[<G_ $conf:upper _PATH>], toml).into_diagnostic()?;
                    }
                )*
            }
        };
    }
    the_configs!(config, cookies, langs, keymap);

    Ok(())
}

/// get the user's config
/// please first use `global_user_config()` for get config
#[instrument]
pub fn get_user_conf() -> Result<User> {
    if !(G_CONFIG_PATH.exists()
        && G_COOKIES_PATH.exists()
        && G_LANGS_PATH.exists()
        && G_KEYMAP_PATH.exists())
    {
        gen_config(Suffix::Com)?;
    }

    let config = fs::read_to_string(&*G_CONFIG_PATH).into_diagnostic()?;
    let mut config: Config = toml::from_str(&config)
        .into_diagnostic()
        .expect(
            "something wrong, you can backup of `config.toml` as `config.toml.bak` for auto \
             generate",
        );
    let urls = Urls::new(config.url_suffix);

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
    let langs = fs::read_to_string(&*G_LANGS_PATH)
        .into_diagnostic()
        .unwrap();
    let langs = toml::from_str(&langs)
        .into_diagnostic()
        .expect(
            "something wrong, you can backup of `langs.toml` as `langs.toml.bak` for auto generate",
        );

    let cookies = fs::read_to_string(&*G_COOKIES_PATH)
        .into_diagnostic()
        .unwrap();
    let cookies = toml::from_str(&cookies)
        .into_diagnostic()
        .expect(
            "something wrong, you can backup of `cookies.toml` as `cookies.toml.bak` for auto \
             generate",
        );

    let mut user = User {
        urls,
        config,
        cookies,
        langs,
        keymap: TuiKeyMap::default(),
    };

    let key = fs::read_to_string(&*G_KEYMAP_PATH)
        .into_diagnostic()
        .unwrap();
    let key: TuiKeyMap = toml::from_str(&key)
        .into_diagnostic()
        .unwrap_or_default();
    user.keymap.add_keymap(key.keymap);

    Ok(user)
}
