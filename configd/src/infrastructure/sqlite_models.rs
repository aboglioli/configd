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
pub struct SqliteAccess {
    pub source: String,
    pub instance: String,
    pub timestamp: DateTime<Utc>,
    pub previous: Option<DateTime<Utc>>,
}

impl SqliteAccess {
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
pub struct SqliteConfig {
    pub id: String,
    pub name: String,
    pub data: JsonValue,
    pub valid: bool,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}

impl SqliteConfig {
    // pub fn from_domain(config: &Config) -> Result<SqliteConfig, Error> {
    //     Ok(SqliteConfig {
    //         id: config.id().to_string(),
    //         name: config.name().to_string(),
    //         data: config.data().into(),
    //         valid: config.is_valid(),
    //         password: config.password().map(ToString::to_string),
    //         accesses: config
    //             .accesses()
    //             .iter()
    //             .map(|access| SqliteAccess {
    //                 source: access.source().to_string(),
    //                 instance: access.instance().to_string(),
    //                 timestamp: *access.timestamp(),
    //                 previous: access.previous().copied(),
    //             })
    //             .collect(),
    //         created_at: *config.timestamps().created_at(),
    //         updated_at: *config.timestamps().updated_at(),
    //         version: config.version().value(),
    //     })
    // }

    pub fn to_domain(self, accesses: Vec<Access>) -> Result<Config, Error> {
        Config::new(
            Id::new(self.id)?,
            self.name,
            self.data.into(),
            self.valid,
            self.password.map(Password::new).transpose()?,
            accesses,
            Timestamps::new(self.created_at, self.updated_at, None)?,
            Version::new(self.version)?,
        )
    }
}

#[derive(FromRow)]
pub struct SqliteSchema {
    pub id: String,
    pub name: String,
    pub root_prop: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}

impl SqliteSchema {
    // pub fn from_domain(schema: &Schema) -> Result<SqliteSchema, Error> {
    //     let configs = serde_json::to_value(
    //         schema
    //             .configs()
    //             .iter()
    //             .map(|(id, config)| Ok((id.to_string(), SqliteConfig::from_domain(config)?)))
    //             .collect::<Result<HashMap<String, SqliteConfig>, Error>>()?,
    //     )
    //     .map_err(Error::Serde)?;
    //
    //     Ok(SqliteSchema {
    //         id: schema.id().to_string(),
    //         name: schema.name().to_string(),
    //         root_prop: schema.root_prop().clone().try_into()?,
    //         configs,
    //         created_at: *schema.timestamps().created_at(),
    //         updated_at: *schema.timestamps().updated_at(),
    //         version: schema.version().value(),
    //     })
    // }

    pub fn to_domain(self, configs: HashMap<Id, Config>) -> Result<Schema, Error> {
        Schema::new(
            Id::new(self.id)?,
            self.name,
            self.root_prop.try_into()?,
            configs,
            Timestamps::new(self.created_at, self.updated_at, None)?,
            Version::new(self.version)?,
            None,
        )
    }
}
