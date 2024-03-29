use chrono::{DateTime, Duration, Utc};

use crate::domain::shared::Id;

#[derive(Debug, Clone)]
pub struct Access {
    source: Id,
    instance: Id,
    timestamp: DateTime<Utc>,
    previous: Option<DateTime<Utc>>,
}

impl Access {
    pub fn new(
        source: Id,
        instance: Id,
        timestamp: DateTime<Utc>,
        previous: Option<DateTime<Utc>>,
    ) -> Access {
        Access {
            source,
            instance,
            timestamp,
            previous,
        }
    }

    pub fn unknown() -> Access {
        Access::new(
            Id::new("unknown").unwrap(),
            Id::new("unknown").unwrap(),
            Utc::now(),
            None,
        )
    }

    pub fn create(source: Id, instance: Id) -> Access {
        Access::new(source, instance, Utc::now(), None)
    }

    pub fn create_with_source(source: Id) -> Access {
        Access::new(source, Id::new("unknown").unwrap(), Utc::now(), None)
    }

    pub fn create_with_instance(instance: Id) -> Access {
        Access::new(Id::new("unknown").unwrap(), instance, Utc::now(), None)
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

    pub fn previous(&self) -> Option<&DateTime<Utc>> {
        self.previous.as_ref()
    }

    pub fn ping(&self) -> Access {
        Access::new(
            self.source.clone(),
            self.instance.clone(),
            Utc::now(),
            Some(self.timestamp),
        )
    }

    pub fn elapsed_time(&self) -> Duration {
        Utc::now() - self.timestamp
    }

    pub fn elapsed_time_from_previous(&self) -> Option<Duration> {
        self.previous.map(|previous| self.timestamp - previous)
    }

    pub fn equals(&self, other: &Access) -> bool {
        self.source == other.source && self.instance == other.instance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::Duration;

    #[test]
    fn create_and_ping() {
        let mut access = Access::create(Id::new("source").unwrap(), Id::new("instance").unwrap());

        assert!(access.previous().is_none());

        access = access.ping();

        assert!(access.previous().is_some());

        let duration = *access.timestamp() - *access.previous().unwrap();
        assert!(duration > Duration::zero());
    }
}
