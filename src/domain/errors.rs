use core_lib::errors::Error as CoreError;
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
    #[error("could not deserialize prop")]
    CouldNotDeserializeProp(#[source] serde_json::Error),
    #[error("invalid array: must have just one element")]
    InvalidArray,
    #[error("root prop is not an object or array")]
    UnknownRootProp,

    // Entities
    #[error("could not record event")]
    CouldNotRecordEvent(#[source] CoreError),
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

    // Config validation
    #[error("invalid config")]
    InvalidConfig(Diff),
}

impl Error {
    pub fn code(&self) -> &str {
        match self {
            Error::EmptyId => "empty_id",
            Error::EmptyName => "empty_name",
            Error::EmptyInterval => "empty_interval",
            Error::MismatchedKinds { .. } => "mismatched_kinds",
            Error::CouldNotDeserializeProp(_) => "could_not_deserialize_prop",
            Error::InvalidArray => "invalid_array",
            Error::UnknownRootProp => "unknown_root_prop",
            Error::CouldNotRecordEvent(_) => "could_not_record_event",
            Error::SchemaNotFound(_) => "schema_not_found",
            Error::SchemaAlreadyExists(_) => "schema_already_exists",
            Error::SchemaContainsConfigs(_) => "schema_contains_configs",
            Error::ConfigNotFound(_) => "config_not_found",
            Error::ConfigAlreadyExists(_) => "config_already_exists",
            Error::InvalidConfig(_) => "invalid_config",
        }
    }
}
