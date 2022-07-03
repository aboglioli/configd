use std::sync::Arc;

use crate::{
    config::{Config, Storage},
    domain::Error,
    infrastructure::InMemSchemaRepository,
};

pub struct Container {
    pub schema_repository: Arc<InMemSchemaRepository>,
}

impl Container {
    pub fn build(config: &Config) -> Result<Container, Error> {
        let schema_repository = Arc::new(match config.storage {
            Storage::InMem => InMemSchemaRepository::new(),
        });

        Ok(Container { schema_repository })
    }
}
