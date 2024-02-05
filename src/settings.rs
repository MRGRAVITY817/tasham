mod database_settings;

use {
    self::database_settings::DatabaseSettings,
    crate::result::{AppError, AppResult},
    config::{Config, Environment, File},
    serde::Deserialize,
    std::env,
};

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn new() -> AppResult<Self> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            .set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("database.url"));

        // Deserialize (and thus freeze) the entire configuration
        s.try_deserialize()
            .map_err(|e| AppError::Config { source: e })
    }
}
