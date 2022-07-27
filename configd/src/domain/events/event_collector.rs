use serde::Serialize;
use std::mem;

use crate::domain::{errors::Error, events::Event};

pub trait Publishable: Serialize {
    fn entity_id(&self) -> &str;
    fn topic(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct EventCollector {
    events: Vec<Event>,
}

impl EventCollector {
    pub fn new(events: Vec<Event>) -> EventCollector {
        EventCollector { events }
    }

    pub fn create() -> EventCollector {
        EventCollector::new(Vec::new())
    }

    pub fn record<P>(&mut self, p: P) -> Result<(), Error>
    where
        P: Publishable,
    {
        let event = Event::create(p.entity_id(), p.topic(), &p)?;

        self.events.push(event);

        Ok(())
    }

    pub fn all(&self) -> &[Event] {
        &self.events
    }

    pub fn drain(&mut self) -> Vec<Event> {
        mem::take(&mut self.events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct SomethingHappened {
        name: String,
    }

    impl Publishable for SomethingHappened {
        fn entity_id(&self) -> &str {
            "something-happened#01"
        }

        fn topic(&self) -> &str {
            "something.happened"
        }
    }

    #[test]
    fn get_all_events() {
        let mut collector = EventCollector::create();

        collector
            .record(SomethingHappened {
                name: "hello".to_string(),
            })
            .unwrap();

        assert_eq!(collector.all().len(), 1);

        let events = collector.drain();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].entity_id(), "something-happened#01");
        assert_eq!(events[0].topic(), "something.happened");

        assert!(collector.drain().is_empty());
    }
}
