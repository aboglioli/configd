use crate::domain::{Error, Id, Value};

#[derive(Debug, Clone)]
pub struct Config {
    id: Id,
    name: String,

    data: Value,
    valid: bool,
}

impl Config {
    pub fn new(id: Id, name: String, data: Value, valid: bool) -> Result<Config, Error> {
        if name.is_empty() {
            return Err(Error::Generic);
        }

        Ok(Config {
            id,
            name,
            data,
            valid,
        })
    }

    pub fn create(id: Id, name: String, data: Value, valid: bool) -> Result<Config, Error> {
        Config::new(Id::slug(&name)?, name, data, valid)
    }

    pub fn id(&self) -> &Id {
        &self.id
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

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn change_data(&mut self, data: Value) -> Result<(), Error> {
        self.data = data;

        Ok(())
    }

    pub fn set_valid(&mut self, valid: bool) {
        self.valid = valid;
    }
}
