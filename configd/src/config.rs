use std::{env, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("invalid environment: {0}")]
    InvalidEnvironment(String),
    #[error("invalid storage: {0}")]
    InvalidStorage(String),
}

// Environment
pub enum Environment {
    Dev,
    Stg,
    Prod,
}

impl FromStr for Environment {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Environment, Self::Err> {
        match s {
            "dev" => Ok(Environment::Dev),
            "stg" => Ok(Environment::Stg),
            "prod" => Ok(Environment::Prod),
            _ => Err(ConfigError::InvalidEnvironment(s.to_string())),
        }
    }
}

// Storage
pub enum Storage {
    InMem,
    SQLite { filename: String },
}

impl FromStr for Storage {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Storage, Self::Err> {
        match s {
            "in-mem" => Ok(Storage::InMem),
            "sqlite" => Ok(Storage::SQLite {
                filename: env::var("SQLITE_FILENAME").unwrap_or("configd.db".to_string()),
            }),
            _ => Err(ConfigError::InvalidStorage(s.to_string())),
        }
    }
}

pub struct Config {
    pub env: Environment,
    pub host: String,
    pub port: u16,
    pub storage: Storage,
}

impl Config {
    pub fn load() -> Result<Config, ConfigError> {
        Ok(Config {
            env: if let Ok(env) = env::var("ENV") {
                Environment::from_str(&env)?
            } else {
                Environment::Dev
            },
            host: env::var("HOST").unwrap_or("127.0.0.1".to_string()),
            port: env::var("PORT")
                .map(|port| port.parse().unwrap())
                .unwrap_or(8080),
            storage: env::var("STORAGE")
                .map(|storage| Storage::from_str(&storage).unwrap())
                .unwrap_or(Storage::InMem),
        })
    }
}
