use chrono::{DateTime, Utc};
use core_lib::models::{Timestamps, Version};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::collections::HashMap;

use crate::domain::{Access, Config, Error, Id, Schema};

#[derive(Serialize, Deserialize)]
pub struct SqliteAccess {
    pub source: String,
    pub instance: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct SqliteConfig {
    pub id: String,
    pub name: String,
    pub data: JsonValue,
    pub valid: bool,
    pub checksum: String,
    pub accesses: Vec<SqliteAccess>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}

impl SqliteConfig {
    pub fn from_domain(config: &Config) -> Result<SqliteConfig, Error> {
        Ok(SqliteConfig {
            id: config.id().to_string(),
            name: config.name().to_string(),
            data: config.data().into(),
            valid: config.is_valid(),
            checksum: config.checksum().to_string(),
            accesses: config
                .accesses()
                .iter()
                .map(|access| SqliteAccess {
                    source: access.source().to_string(),
                    instance: access.instance().to_string(),
                    timestamp: *access.timestamp(),
                })
                .collect(),
            created_at: *config.timestamps().created_at(),
            updated_at: *config.timestamps().updated_at(),
            version: config.version().value(),
        })
    }

    pub fn to_domain(self) -> Result<Config, Error> {
        Config::new(
            Id::new(self.id)?,
            self.name,
            self.data.into(),
            self.valid,
            self.checksum,
            self.accesses
                .into_iter()
                .map(|access| {
                    Ok(Access::new(
                        Id::new(access.source)?,
                        Id::new(access.instance)?,
                        access.timestamp,
                    ))
                })
                .collect::<Result<Vec<Access>, Error>>()?,
            Timestamps::new(self.created_at, self.updated_at, None).map_err(Error::Core)?,
            Version::new(self.version).map_err(Error::Core)?,
        )
    }
}

#[derive(FromRow)]
pub struct SqliteSchema {
    pub id: String,
    pub name: String,
    pub root_prop: JsonValue,
    pub configs: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}

impl SqliteSchema {
    pub fn from_domain(schema: &Schema) -> Result<SqliteSchema, Error> {
        let configs = serde_json::to_value(
            schema
                .configs()
                .iter()
                .map(|(id, config)| Ok((id.to_string(), SqliteConfig::from_domain(config)?)))
                .collect::<Result<HashMap<String, SqliteConfig>, Error>>()?,
        )
        .map_err(Error::Serde)?;

        Ok(SqliteSchema {
            id: schema.id().to_string(),
            name: schema.name().to_string(),
            root_prop: schema.root_prop().clone().try_into()?,
            configs,
            created_at: *schema.timestamps().created_at(),
            updated_at: *schema.timestamps().updated_at(),
            version: schema.version().value(),
        })
    }

    pub fn to_domain(self) -> Result<Schema, Error> {
        let configs: HashMap<String, SqliteConfig> =
            serde_json::from_value(self.configs).map_err(Error::Serde)?;

        Schema::new(
            Id::new(self.id)?,
            self.name,
            self.root_prop.try_into()?,
            configs
                .into_iter()
                .map(|(id, config)| Ok((Id::new(id)?, config.to_domain()?)))
                .collect::<Result<HashMap<Id, Config>, Error>>()?,
            Timestamps::new(self.created_at, self.updated_at, None).map_err(Error::Core)?,
            Version::new(self.version).map_err(Error::Core)?,
            None,
        )
    }
}
