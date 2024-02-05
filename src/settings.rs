pub mod database_settings;

use {self::database_settings::DatabaseSettings, crate::result::AppResult, serde::Deserialize};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn new() -> AppResult<Self> {
        Ok(Self {
            database: DatabaseSettings::new()?,
        })
    }
}
