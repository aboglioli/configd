use async_trait::async_trait;

use crate::domain::{Error, Id, Value};

#[async_trait]
pub trait ConfigRepository {
    async fn find_by_id(&self, schema_id: &Id, config_id: &Id) -> Result<Option<Config>, Error>;
    async fn exists(&self, schema_id: &Id, id: &Id) -> Result<bool, Error>;
    async fn save(&self, config: &mut Config) -> Result<(), Error>;
    async fn delete(&self, schema_id: &Id, id: &Id) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct Config {
    id: Id,
    schema_id: Id,
    name: String,

    data: Value,
    valid: Option<bool>,
}

impl Config {
    pub fn new(
        id: Id,
        schema_id: Id,
        name: String,
        data: Value,
        valid: Option<bool>,
    ) -> Result<Config, Error> {
        if name.is_empty() {
            return Err(Error::Generic);
        }

        Ok(Config {
            id,
            schema_id,
            name,
            data,
            valid,
        })
    }

    pub fn create(schema_id: Id, name: String, data: Value) -> Result<Config, Error> {
        Config::new(schema_id, Id::slug(&name)?, name, data, None)
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn schema_id(&self) -> &Id {
        &self.schema_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> &Value {
        &self.data
    }

    pub fn into_data(self) -> Value {
        self.data
    }

    pub fn is_valid(&self) -> Option<bool> {
        self.valid
    }

    pub fn change_data(&mut self, data: Value) -> Result<(), Error> {
        self.data = data;

        Ok(())
    }

    pub fn mark_as_valid(&mut self) {
        self.valid = Some(true);
    }

    pub fn mark_as_invalid(&mut self) {
        self.valid = Some(false);
    }
}
