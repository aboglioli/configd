use core_lib::models::{Timestamps, Version};

use crate::domain::{Error, Id, Value};

#[derive(Debug, Clone)]
pub struct Config {
    id: Id,
    name: String,

    data: Value,
    valid: bool,
    checksum: Vec<u8>,

    timestamps: Timestamps,
    version: Version,
}

impl Config {
    pub fn new(
        id: Id,
        name: String,
        data: Value,
        valid: bool,
        checksum: Vec<u8>,
        timestamps: Timestamps,
        version: Version,
    ) -> Result<Config, Error> {
        if name.is_empty() {
            return Err(Error::EmptyName);
        }

        Ok(Config {
            id,
            name,
            data,
            valid,
            checksum,
            timestamps,
            version,
        })
    }

    pub fn create(
        id: Id,
        name: String,
        data: Value,
        valid: bool,
        checksum: Vec<u8>,
    ) -> Result<Config, Error> {
        Config::new(
            id,
            name,
            data,
            valid,
            checksum,
            Timestamps::create(),
            Version::init_version(),
        )
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

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn checksum(&self) -> &[u8] {
        &self.checksum
    }

    pub fn timestamps(&self) -> &Timestamps {
        &self.timestamps
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn change_data(&mut self, data: Value, valid: bool) -> Result<(), Error> {
        self.data = data;
        self.valid = valid;

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();

        Ok(())
    }

    pub fn mark_as_invalid(&mut self) {
        self.valid = false;

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();
    }
}
