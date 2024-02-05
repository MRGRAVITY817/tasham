use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub username: String,
    pub password: String,
}
