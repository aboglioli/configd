use chrono::Duration;

use crate::domain::{
    configs::{Access, Password},
    errors::Error,
    shared::{Id, Timestamps, Version},
    values::Value,
};

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

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();

        Ok(())
    }

    pub fn delete_password(&mut self, password: Option<&Password>) -> Result<(), Error> {
        if !self.can_access(password) {
            return Err(Error::Unauthorized);
        }

        self.password = None;

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();

        Ok(())
    }

    pub fn register_access(&mut self, access: Access) -> &Access {
        let index: usize;
        if let Some((i, access)) = self
            .accesses
            .iter_mut()
            .enumerate()
            .find(|(_, a)| a.equals(&access))
        {
            *access = access.ping();
            index = i;
        } else {
            index = self.accesses.len();
            self.accesses.push(access);
        }

        &self.accesses[index]
    }

    pub fn clean_old_accesses(&mut self) -> Vec<Access> {
        let access_indexes_to_remove: Vec<usize> = self
            .accesses
            .iter()
            .enumerate()
            .filter(|(_, access)| {
                let max_duration = access
                    .elapsed_time_from_previous()
                    .map(|previous| {
                        if previous.num_seconds() < 2 {
                            previous + Duration::seconds(2)
                        } else {
                            previous * 2
                        }
                    })
                    .unwrap_or_else(|| Duration::seconds(30));

                access.elapsed_time() > max_duration
            })
            .map(|(i, _)| i)
            .collect();

        access_indexes_to_remove
            .into_iter()
            .rev() // reverse to not re-calculate indexes
            .map(|i| self.accesses.remove(i))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{DateTime, Utc};

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
        config.register_access(Access::create(
            Id::new("Source 1").unwrap(),
            Id::new("instance#01").unwrap(),
        ));
        config.register_access(Access::create(
            Id::new("Source 2").unwrap(),
            Id::new("instance#01").unwrap(),
        ));

        assert_eq!(config.accesses()[0].source().value(), "Source 1");
        assert_eq!(config.accesses()[1].source().value(), "Source 2");

        // Existing source
        config.register_access(Access::create(
            Id::new("Source 1").unwrap(),
            Id::new("instance#01").unwrap(),
        ));

        assert_eq!(config.accesses()[0].source().value(), "Source 1");
        assert_eq!(config.accesses()[1].source().value(), "Source 2");

        // New source
        config.register_access(Access::create(
            Id::new("Source 3").unwrap(),
            Id::new("instance#01").unwrap(),
        ));

        assert_eq!(config.accesses().len(), 3);
        assert_eq!(config.accesses()[0].source().value(), "Source 1");
        assert_eq!(config.accesses()[1].source().value(), "Source 2");
        assert_eq!(config.accesses()[2].source().value(), "Source 3");

        // New instance
        config.register_access(Access::create(
            Id::new("Source 1").unwrap(),
            Id::new("instance#02").unwrap(),
        ));

        assert_eq!(config.accesses().len(), 4);
        assert_eq!(config.accesses()[0].source().value(), "Source 1");
        assert_eq!(config.accesses()[0].instance().value(), "instance#01");
        assert_eq!(config.accesses()[1].source().value(), "Source 2");
        assert_eq!(config.accesses()[2].source().value(), "Source 3");
        assert_eq!(config.accesses()[3].source().value(), "Source 1");
        assert_eq!(config.accesses()[3].instance().value(), "instance#02");
    }

    #[test]
    fn clean_old_accesses() {
        let mut config = Config::create(
            Id::new("config#01").unwrap(),
            "Config".to_string(),
            Value::String("data".to_string()),
            true,
            None,
        )
        .unwrap();

        config.register_access(Access::new(
            Id::new("Source 1").unwrap(),
            Id::new("instance#01").unwrap(),
            DateTime::parse_from_rfc3339("2022-07-25T19:00:00Z")
                .unwrap()
                .into(),
            None,
        ));

        config.register_access(Access::new(
            Id::new("Source 2").unwrap(),
            Id::new("instance#01").unwrap(),
            DateTime::parse_from_rfc3339("2022-07-25T19:30:00Z")
                .unwrap()
                .into(),
            None,
        ));

        config.register_access(Access::new(
            Id::new("Source 1").unwrap(),
            Id::new("instance#02").unwrap(),
            Utc::now(),
            None,
        ));

        let removed_accesses = config.clean_old_accesses();

        assert_eq!(removed_accesses.len(), 2);
        assert_eq!(config.accesses().len(), 1);
        assert_eq!(config.accesses()[0].source().value(), "Source 1");
        assert_eq!(config.accesses()[0].instance().value(), "instance#02");
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
