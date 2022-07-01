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
