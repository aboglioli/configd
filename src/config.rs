use std::{env, str::FromStr};

use crate::domain::Error;

// Environment
pub enum Environment {
    Dev,
    Stg,
    Prod,
}

impl FromStr for Environment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Environment, Error> {
        match s {
            "dev" => Ok(Environment::Dev),
            "stg" => Ok(Environment::Stg),
            "prod" => Ok(Environment::Prod),
            _ => Err(Error::Generic),
        }
    }
}

// Storage
pub enum Storage {
    InMem,
}

impl FromStr for Storage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Storage, Error> {
        match s {
            "in-mem" => Ok(Storage::InMem),
            _ => Err(Error::Generic),
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
    pub fn load() -> Result<Config, Error> {
        Ok(Config {
            env: if let Ok(env) = env::var("ENV") {
                Environment::from_str(&env)?
            } else {
                Environment::Dev
            },
            host: env::var("HOST").unwrap_or("127.0.0.1".to_string()),
            port: if let Ok(port) = env::var("PORT") {
                port.parse().map_err(|_| Error::Generic)?
            } else {
                8080
            },
            storage: if let Ok(storage) = env::var("STORAGE") {
                Storage::from_str(&storage)?
            } else {
                Storage::InMem
            },
        })
    }
}
