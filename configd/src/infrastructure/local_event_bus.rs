use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{
    errors::Error,
    events::{Event, Handler, Publisher, Subscriber},
};

struct Subscription {
    subject: String,
    handler: Box<dyn Handler>,
}

#[derive(Clone)]
pub struct LocalEventBus {
    subscriptions: Arc<RwLock<Vec<Subscription>>>,
}

impl LocalEventBus {
    pub fn new() -> LocalEventBus {
        LocalEventBus {
            subscriptions: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

fn subject_has_topic(subject: &str, topic: &str) -> bool {
    if subject == topic {
        return true;
    }

    let subject_parts: Vec<String> = subject.split('.').map(str::to_lowercase).collect();
    let topic_parts: Vec<String> = topic.split('.').map(str::to_lowercase).collect();

    if subject_parts.len() != topic_parts.len() {
        return false;
    }

    subject_parts.iter().enumerate().all(|(i, subject_part)| {
        let topic_part = &topic_parts[i];

        subject_part == "*" || subject_part == topic_part
    })
}

#[async_trait]
impl Publisher for LocalEventBus {
    async fn publish(&self, events: &[Event]) -> Result<(), Error> {
        let subscriptions = self.subscriptions.read().await;

        for event in events {
            for subscription in subscriptions.iter() {
                if subject_has_topic(&subscription.subject, event.topic()) {
                    subscription.handler.handle(event).await?;
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl Subscriber for LocalEventBus {
    async fn subscribe(&self, subject: &str, handler: Box<dyn Handler>) -> Result<(), Error> {
        let mut subscriptions = self.subscriptions.write().await;

        subscriptions.push(Subscription {
            subject: subject.to_string(),
            handler,
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[derive(Clone)]
    struct Counter {
        count: Arc<Mutex<i64>>,
    }

    impl Counter {
        fn new() -> Counter {
            Counter {
                count: Arc::new(Mutex::new(0)),
            }
        }
    }

    #[async_trait]
    impl Handler for Counter {
        async fn handle(&self, event: &Event) -> Result<(), Error> {
            let incr: i64 = event.deserialize_payload().unwrap();

            let mut count = self.count.lock().await;
            *count += incr;

            Ok(())
        }
    }

    #[tokio::test]
    async fn subscribe_and_publish() {
        let event_bus = LocalEventBus::new();
        let counter = Counter::new();

        // Subscriptions
        let res = event_bus
            .subscribe("topic.*", Box::new(counter.clone()))
            .await;
        assert!(res.is_ok());

        let res = event_bus
            .subscribe("topic.*", Box::new(counter.clone()))
            .await;
        assert!(res.is_ok());

        // Publish
        let event = Event::create("entity#01", "topic.code", &1).unwrap();

        let res = event_bus.publish(&[event]).await;
        assert!(res.is_ok());

        assert_eq!(*counter.count.lock().await, 2);

        let event = Event::create("entity#02", "topic.code", &2).unwrap();

        let res = event_bus.publish(&[event]).await;
        assert!(res.is_ok());

        assert_eq!(*counter.count.lock().await, 6);
    }

    #[tokio::test]
    async fn thread_safe() {
        let event_bus = LocalEventBus::new();
        let counter = Counter::new();
        let event = Event::create("entity#01", "increment.code", &1).unwrap();

        event_bus
            .subscribe("increment.*", Box::new(counter.clone()))
            .await
            .unwrap();

        let event1 = event.clone();
        let event_bus1 = event_bus.clone();
        let t1 = tokio::spawn(async move {
            event_bus1.publish(&[event1]).await.unwrap();
        });

        let event2 = event.clone();
        let event_bus2 = event_bus.clone();
        let t2 = tokio::spawn(async move {
            event_bus2.publish(&[event2]).await.unwrap();
        });

        let (_, _) = tokio::join!(t1, t2);

        assert_eq!(*counter.count.lock().await, 2);
    }
}
