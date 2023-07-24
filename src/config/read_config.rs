use super::User;
use miette::{Error, IntoDiagnostic};
use tokio::{
    fs::{create_dir_all, write, OpenOptions},
    io::AsyncReadExt,
};

/// generate default config
///
/// * `force`: when true will override your config
pub async fn gen_default_conf(force: bool) -> Result<(), Error> {
    let df_user = User::default();
    let config_dir = super::init_config_dir();
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
pub async fn get_user_conf() -> Result<User, Error> {
    let config_dir = super::init_config_dir();
    let mut cf = OpenOptions::new()
        .read(true)
        .open(&config_dir)
        .await
        .into_diagnostic()?;

    let mut content: String = String::new();
    cf.read_to_string(&mut content)
        .await
        .into_diagnostic()?;
    let user: User = toml::from_str(&content).into_diagnostic()?;
    #[cfg(debug_assertions)]
    dbg!(&user);
    Ok(user)
}
