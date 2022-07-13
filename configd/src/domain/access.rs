use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Access {
    source: String,
    timestamp: DateTime<Utc>,
}

impl Access {
    pub fn new(source: String, timestamp: DateTime<Utc>) -> Access {
        Access { source, timestamp }
    }

    pub fn create(source: String) -> Access {
        Access::new(source, Utc::now())
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}
