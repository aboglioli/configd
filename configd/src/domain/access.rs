use chrono::{DateTime, Utc};

use crate::domain::Id;

#[derive(Debug, Clone)]
pub struct Access {
    source: Id,
    instance: Id,
    timestamp: DateTime<Utc>,
}

impl Access {
    pub fn new(source: Id, instance: Id, timestamp: DateTime<Utc>) -> Access {
        Access {
            source,
            instance,
            timestamp,
        }
    }

    pub fn create(source: Id, instance: Id) -> Access {
        Access::new(source, instance, Utc::now())
    }

    pub fn source(&self) -> &Id {
        &self.source
    }

    pub fn instance(&self) -> &Id {
        &self.instance
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}
