use thiserror::Error;

use crate::domain::{Diff, Id, Kind};

#[derive(Error, Debug)]
pub enum Error {
    // General properties
    #[error("empty id")]
    EmptyId,
    #[error("empty name")]
    EmptyName,
    #[error("empty interval")]
    EmptyInterval,

    // Props
    #[error("mismatched kinds: expected {expected}, found {found}")]
    MismatchedKinds { expected: Kind, found: Kind },
    #[error("invalid array: must have just one element")]
    InvalidArray,
    #[error("root prop is not an object or array")]
    UnknownRootProp,

    // Domain & Entities
    #[error("schema not found: {0}")]
    SchemaNotFound(Id),
    #[error("schema already exists: {0}")]
    SchemaAlreadyExists(Id),
    #[error("schema contains configs: {0}")]
    SchemaContainsConfigs(Id),
    #[error("config not found: {0}")]
    ConfigNotFound(Id),
    #[error("config already exists: {0}")]
    ConfigAlreadyExists(Id),
    #[error("page out of range")]
    PageOutOfRange,

    // Config validation
    #[error("invalid config")]
    InvalidConfig(Diff),

    // External
    #[error("serde: {0}")]
    Serde(#[source] serde_json::Error),
    #[error("core lib: {0}")]
    Core(#[source] core_lib::errors::Error),
    #[error("database error: {0}")]
    Database(#[source] sqlx::Error),
}

impl Error {
    pub fn code(&self) -> &str {
        match self {
            Error::EmptyId => "empty_id",
            Error::EmptyName => "empty_name",
            Error::EmptyInterval => "empty_interval",

            Error::MismatchedKinds { .. } => "mismatched_kinds",
            Error::InvalidArray => "invalid_array",
            Error::UnknownRootProp => "unknown_root_prop",

            Error::SchemaNotFound(_) => "schema_not_found",
            Error::SchemaAlreadyExists(_) => "schema_already_exists",
            Error::SchemaContainsConfigs(_) => "schema_contains_configs",
            Error::ConfigNotFound(_) => "config_not_found",
            Error::ConfigAlreadyExists(_) => "config_already_exists",
            Error::PageOutOfRange => "page_out_of_range",

            Error::InvalidConfig(_) => "invalid_config",

            Error::Serde(_) => "serde",
            Error::Core(_) => "core_lib",
            Error::Database(_) => "database",
        }
    }
}
