use chrono::Duration;
use core_lib::models::{Timestamps, Version};

use crate::domain::{Access, Error, Id, Password, Value};

#[derive(Debug, Clone)]
pub struct Config {
    id: Id,
    name: String,

    data: Value,
    valid: bool,
    password: Option<Password>,

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
        password: Option<Password>,
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
            password,
            data,
            valid,
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
        password: Option<Password>,
    ) -> Result<Config, Error> {
        Config::new(
            id,
            name,
            data,
            valid,
            password.map(|password| password.hash()).transpose()?,
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

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn password(&self) -> Option<&Password> {
        self.password.as_ref()
    }

    pub fn can_access(&self, raw_password: Option<&Password>) -> bool {
        if let Some(password) = &self.password {
            if let Some(raw_password) = raw_password {
                password.compare(raw_password)
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn accesses(&self) -> &[Access] {
        &self.accesses
    }

    pub fn timestamps(&self) -> &Timestamps {
        &self.timestamps
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    // Mutations
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

    pub fn change_password(
        &mut self,
        old_password: Option<&Password>,
        new_password: Password,
    ) -> Result<(), Error> {
        if !self.can_access(old_password) {
            return Err(Error::Unauthorized);
        }

        self.password = Some(new_password.hash()?);

        Ok(())
    }

    pub fn delete_password(&mut self, password: Option<&Password>) -> Result<(), Error> {
        if !self.can_access(password) {
            return Err(Error::Unauthorized);
        }

        self.password = None;

        Ok(())
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

        self.accesses.retain(|access| {
            let max_duration = access
                .elapsed_time_from_previous()
                .map(|mut previous| {
                    previous = previous * 2;

                    if previous.num_seconds() < 2 {
                        previous = previous + Duration::seconds(1);
                    }

                    previous
                })
                .unwrap_or_else(|| Duration::seconds(30));

            access.elapsed_time() <= max_duration
        });

        self.accesses.truncate(6);
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
            None,
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

    #[test]
    fn can_access() {
        // No password
        let config = Config::create(
            Id::new("config#01").unwrap(),
            "Config".to_string(),
            Value::String("data".to_string()),
            true,
            None,
        )
        .unwrap();

        assert!(config.can_access(Some(&Password::new("passwd123".to_string()).unwrap())));
        assert!(config.can_access(Some(&Password::new("passwd321".to_string()).unwrap())));

        // With password
        let config = Config::create(
            Id::new("config#01").unwrap(),
            "Config".to_string(),
            Value::String("data".to_string()),
            true,
            Some(Password::new("passwd123".to_string()).unwrap()),
        )
        .unwrap();

        assert_ne!(config.password().unwrap().value(), "passwd123");
        assert!(config.can_access(Some(&Password::new("passwd123".to_string()).unwrap())));
        assert!(!config.can_access(Some(&Password::new("passwd321".to_string()).unwrap())));
    }
}
