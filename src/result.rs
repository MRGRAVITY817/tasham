use std::fmt::Display;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum AppError {
    Db {
        #[from]
        source: surrealdb::Error,
    },
    Config {
        #[from]
        source: config::ConfigError,
    },
    Env {
        #[from]
        source: std::env::VarError,
    },
    Io {
        #[from]
        source: std::io::Error,
    },
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Db { source } => write!(f, "{}", source),
            AppError::Config { source } => write!(f, "{}", source),
            AppError::Env { source } => write!(f, "{}", source),
            AppError::Io { source } => write!(f, "{}", source),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
