use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::collections::HashMap;

use crate::domain::{
    configs::{Access, Config, Password},
    errors::Error,
    schemas::Schema,
    shared::{Id, Timestamps, Version},
};

#[derive(FromRow)]
pub struct SqlxAccess {
    pub source: String,
    pub instance: String,
    pub timestamp: DateTime<Utc>,
    pub previous: Option<DateTime<Utc>>,
}

impl SqlxAccess {
    pub fn to_domain(self) -> Result<Access, Error> {
        Ok(Access::new(
            Id::new(self.source)?,
            Id::new(self.instance)?,
            self.timestamp,
            self.previous,
        ))
    }
}

#[derive(FromRow)]
pub struct SqlxConfig {
    pub id: String,
    pub name: String,
    pub data: JsonValue,
    pub valid: bool,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
}

impl SqlxConfig {
    pub fn to_domain(self, accesses: Vec<Access>) -> Result<Config, Error> {
        Config::new(
            Id::new(self.id)?,
            self.name,
            self.data.into(),
            self.valid,
            self.password.map(Password::new).transpose()?,
            accesses,
            Timestamps::new(self.created_at, self.updated_at, None)?,
            Version::new(self.version.into())?,
        )
    }
}

#[derive(FromRow)]
pub struct SqlxSchema {
    pub id: String,
    pub name: String,
    pub root_prop: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
}

impl SqlxSchema {
    pub fn to_domain(self, configs: HashMap<Id, Config>) -> Result<Schema, Error> {
        Schema::new(
            Id::new(self.id)?,
            self.name,
            self.root_prop.try_into()?,
            configs,
            Timestamps::new(self.created_at, self.updated_at, None)?,
            Version::new(self.version.into())?,
            None,
        )
    }
}
