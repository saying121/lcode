use std::fs::{self, write, OpenOptions};

use miette::{Context, IntoDiagnostic, Result};

use super::{global::*, user_nested::Suffix, LcodeConfig};
use crate::{
    config::{user_nested::Urls, Config},
    keymap::TuiKeyMap,
};

impl LcodeConfig {
    /// generate config
    pub fn gen_config(suffix: Suffix) -> Result<()> {
        let user = Self::new(suffix);

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
    pub fn get_user_conf() -> Result<Self> {
        let config = fs::read_to_string(&*G_CONFIG_PATH).unwrap_or_else(|err| {
            tracing::info!("no config.toml: {err}");
            String::new()
        });
        let mut config: Config = toml::from_str(&config).into_diagnostic()?;
        let urls = Urls::new(config.url_suffix);

        if config.code_dir.starts_with("~") {
            let mut path = config
                .code_dir
                .to_string_lossy()
                .to_string();
            let path = path.split_off(2);
            let mut code_dir = dirs::home_dir().expect("get home_dir failed");
            code_dir.push(path);
            config.code_dir = code_dir;
        }
        let langs = fs::read_to_string(&*G_LANGS_PATH).unwrap_or_else(|err| {
            tracing::info!("no langs.toml: {err}");
            String::new()
        });
        let langs = toml::from_str(&langs).into_diagnostic()?;

        let cookies = fs::read_to_string(&*G_COOKIES_PATH).unwrap_or_else(|err| {
            tracing::info!("no config.toml: {err}");
            String::new()
        });
        let cookies = toml::from_str(&cookies).unwrap_or_default();

        let mut user = Self {
            urls,
            config,
            cookies,
            langs,
            keymap: TuiKeyMap::default(),
        };

        let key = fs::read_to_string(&*G_KEYMAP_PATH).unwrap_or_else(|err| {
            tracing::info!("no keymap.toml: {err}");
            String::new()
        });
        let key: TuiKeyMap = toml::from_str(&key)
            .into_diagnostic()
            .context("get keymap failed")?;
        user.keymap.add_keymap(key.map_set);

        Ok(user)
    }
}
