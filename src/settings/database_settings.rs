use std::env;

use crate::result::AppResult;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl DatabaseSettings {
    pub fn new() -> AppResult<Self> {
        Ok(Self {
            url: env::var("SURREAL_URL")?,
            username: env::var("SURREAL_USER")?,
            password: env::var("SURREAL_PASS")?,
        })
    }
}

pub type AppDb = Surreal<Client>;
