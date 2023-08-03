use crate::config::user_nest::Cookies;

use super::User;
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
    let s: toml::Value = toml::from_str(&content).into_diagnostic()?;
    trace!("user config toml Value: {:#?}", s);

    let user: User = User {
        cookie: s
            .get("cookie")
            .and_then(|v| v.as_table())
            .map(|v| Cookies {
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
            })
            .unwrap_or_default(),

        editor: s
            .get("editor")
            .and_then(|v| v.as_array())
            .map(|v| {
                v.iter()
                    .map(|v| {
                        v.as_str()
                            .unwrap_or_default()
                            .to_string()
                    })
                    .collect()
            })
            .unwrap_or_else(|| {
                warn!("user config parser editor error, use vim");
                vec!["vim".to_string()]
            }),
        lang: s
            .get("lang")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                warn!("user config parser lang error, use rust");
                "rust"
            })
            .to_string(),
        code_dir: s
            .get("code_dir")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                warn!("user config parser code_dir error, use default");
                ""
            })
            .into(),
        favorites: s
            .get("favorites")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                warn!("user config parser favorites error, use default");
                "https://leetcode.com/list/api/questions"
            })
            .to_string(),
        origin_url: s
            .get("origin_url")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                warn!("user config parser origin_url error, use default");
                "https://leetcode.com"
            })
            .to_string(),
        graphql: s
            .get("graphql")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                warn!("user config parser graphql error, use default");
                "https://leetcode.com/graphql"
            })
            .to_string(),
        all_problem_api: s
            .get("all_problem_api")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                warn!("user config parser all_problem_api error, use default");
                "https://leetcode.cn/api/problems/$category"
            })
            .to_string(),
        submit: s
            .get("submit")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                warn!("user config parser submit error, use default");
                "https://leetcode.cn/problems/$slug/submit/"
            })
            .to_string(),
        test: s
            .get("test")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                warn!("user config parser test error, use default");
                "https://leetcode.cn/problems/$slug/interpret_solution/"
            })
            .to_string(),
        submissions: s
            .get("submissions")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                warn!("user config parser submissions error, use default");
                "https://leetcode.cn/problems/$slug/interpret_solution/"
            })
            .to_string(),
        ..Default::default()
    };

    // let user = toml::from_str(&content)
    //     .map_err(|err| miette!("Error: {}, parser user config failed", err));
    // let user: User = match user {
    //     Ok(v) => v,
    //     Err(e) => {
    //         warn!("{}, use default config.", e);
    //         User::new("default")
    //     }
    // };

    trace!("the get user config: {:#?}", user);

    Ok(user)
}
