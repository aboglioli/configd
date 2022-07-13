use core_lib::models::{Timestamps, Version};

use crate::domain::{Access, Error, Id, Value};

#[derive(Debug, Clone)]
pub struct Config {
    id: Id,
    name: String,

    data: Value,
    valid: bool,
    checksum: Vec<u8>,

    accesses: Vec<Access>,

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
        accesses: Vec<Access>,
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
            accesses,
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
            Vec::new(),
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

    pub fn change_data(
        &mut self,
        data: Value,
        valid: bool,
        checksum: Vec<u8>,
    ) -> Result<(), Error> {
        self.data = data;
        self.valid = valid;
        self.checksum = checksum;

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();

        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn mark_as_invalid(&mut self) {
        self.valid = false;

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();
    }

    pub fn checksum(&self) -> &[u8] {
        &self.checksum
    }

    pub fn accesses(&self) -> &[Access] {
        &self.accesses
    }

    pub fn register_access(&mut self, source: String) {
        if let Some(access) = self
            .accesses
            .iter_mut()
            .find(|access| access.source() == source)
        {
            *access = Access::create(source);
        } else {
            self.accesses.push(Access::create(source));
        }

        self.accesses
            .sort_by(|access1, access2| access2.timestamp().cmp(access1.timestamp()));
        self.accesses.truncate(6);
    }

    pub fn timestamps(&self) -> &Timestamps {
        &self.timestamps
    }

    pub fn version(&self) -> &Version {
        &self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_access() {
        let mut config = Config::create(
            Id::new("config#01").unwrap(),
            "Config".to_string(),
            Value::String("data".to_string()),
            true,
            vec![1, 2, 3, 4],
        )
        .unwrap();

        // New sources
        config.register_access("Source 1".to_string());
        config.register_access("Source 2".to_string());

        assert_eq!(config.accesses()[0].source(), "Source 2");
        assert_eq!(config.accesses()[1].source(), "Source 1");

        // Existing source
        config.register_access("Source 1".to_string());

        assert_eq!(config.accesses()[0].source(), "Source 1");
        assert_eq!(config.accesses()[1].source(), "Source 2");

        // New source
        config.register_access("Source 3".to_string());

        assert_eq!(config.accesses().len(), 3);
        assert_eq!(config.accesses()[0].source(), "Source 3");
        assert_eq!(config.accesses()[1].source(), "Source 1");
        assert_eq!(config.accesses()[2].source(), "Source 2");

        // Save last accesses only

        config.register_access("Source 4".to_string());
        config.register_access("Source 5".to_string());
        config.register_access("Source 6".to_string());
        config.register_access("Source 7".to_string());

        assert_eq!(config.accesses().len(), 6);
        assert_eq!(config.accesses()[0].source(), "Source 7");
        assert_eq!(config.accesses()[1].source(), "Source 6");
        assert_eq!(config.accesses()[2].source(), "Source 5");
        assert_eq!(config.accesses()[3].source(), "Source 4");
        assert_eq!(config.accesses()[4].source(), "Source 3");
        assert_eq!(config.accesses()[5].source(), "Source 1");
    }
}
