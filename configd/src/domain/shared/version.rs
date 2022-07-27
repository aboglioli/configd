use crate::domain::errors::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Version {
    version: i64,
    updated: bool,
}

impl Version {
    pub fn new(version: i64) -> Result<Version, Error> {
        if version < 1 {
            return Err(Error::InvalidVersion);
        }

        Ok(Version {
            version,
            updated: false,
        })
    }

    pub fn init_version() -> Version {
        Version {
            version: 1,
            updated: true,
        }
    }

    pub fn value(&self) -> i64 {
        self.version
    }

    pub fn incr(&self) -> Version {
        if self.updated {
            return self.clone();
        }

        Version {
            version: self.version + 1,
            updated: true,
        }
    }
}
