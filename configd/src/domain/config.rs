use core_lib::models::{Timestamps, Version};

use crate::domain::{Access, Error, Id, Value};

#[derive(Debug, Clone)]
pub struct Config {
    id: Id,
    name: String,

    data: Value,
    valid: bool,
    checksum: String,

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
        checksum: String,
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
        checksum: String,
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

    pub fn change_data(&mut self, data: Value, valid: bool, checksum: String) -> Result<(), Error> {
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

    pub fn checksum(&self) -> &str {
        &self.checksum
    }

    pub fn accesses(&self) -> &[Access] {
        &self.accesses
    }

    pub fn register_access(&mut self, source: Id, instance: Id) {
        if let Some(access) = self
            .accesses
            .iter_mut()
            .find(|access| access.source() == &source && access.instance() == &instance)
        {
            *access = access.ping();
        } else {
            self.accesses.push(Access::create(source, instance));
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
            "abcd1234".to_string(),
        )
        .unwrap();

        // New sources
        config.register_access(
            Id::new("Source 1").unwrap(),
            Id::new("instance#01").unwrap(),
        );
        config.register_access(
            Id::new("Source 2").unwrap(),
            Id::new("instance#01").unwrap(),
        );

        assert_eq!(config.accesses()[0].source().value(), "Source 2");
        assert_eq!(config.accesses()[1].source().value(), "Source 1");

        // Existing source
        config.register_access(
            Id::new("Source 1").unwrap(),
            Id::new("instance#01").unwrap(),
        );

        assert_eq!(config.accesses()[0].source().value(), "Source 1");
        assert_eq!(config.accesses()[1].source().value(), "Source 2");

        // New source
        config.register_access(
            Id::new("Source 3").unwrap(),
            Id::new("instance#01").unwrap(),
        );

        assert_eq!(config.accesses().len(), 3);
        assert_eq!(config.accesses()[0].source().value(), "Source 3");
        assert_eq!(config.accesses()[1].source().value(), "Source 1");
        assert_eq!(config.accesses()[2].source().value(), "Source 2");

        // Save last accesses only

        config.register_access(
            Id::new("Source 4").unwrap(),
            Id::new("instance#01").unwrap(),
        );
        config.register_access(
            Id::new("Source 5").unwrap(),
            Id::new("instance#01").unwrap(),
        );
        config.register_access(
            Id::new("Source 6").unwrap(),
            Id::new("instance#01").unwrap(),
        );
        config.register_access(
            Id::new("Source 7").unwrap(),
            Id::new("instance#01").unwrap(),
        );

        assert_eq!(config.accesses().len(), 6);
        assert_eq!(config.accesses()[0].source().value(), "Source 7");
        assert_eq!(config.accesses()[1].source().value(), "Source 6");
        assert_eq!(config.accesses()[2].source().value(), "Source 5");
        assert_eq!(config.accesses()[3].source().value(), "Source 4");
        assert_eq!(config.accesses()[4].source().value(), "Source 3");
        assert_eq!(config.accesses()[5].source().value(), "Source 1");
    }
}
