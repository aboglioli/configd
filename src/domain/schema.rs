use async_trait::async_trait;
use core_lib::{
    events::{Event, EventCollector},
    models::{Timestamps, Version},
};
use std::collections::HashMap;

use crate::domain::{
    Config, ConfigAccessed, ConfigCreated, ConfigDataChanged, ConfigDeleted, Error, Id, Prop,
    SchemaCreated, SchemaDeleted, SchemaRootPropChanged, Value,
};

#[async_trait]
pub trait SchemaRepository {
    async fn find_by_id(&self, id: &Id) -> Result<Option<Schema>, Error>;
    async fn exists(&self, id: &Id) -> Result<bool, Error>;
    async fn save(&self, schema: &mut Schema) -> Result<(), Error>;
    async fn delete(&self, id: &Id) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct Schema {
    id: Id,
    name: String,

    root_prop: Prop,

    configs: HashMap<Id, Config>,

    timestamps: Timestamps,
    version: Version,

    event_collector: EventCollector,
}

impl Schema {
    pub fn new(
        id: Id,
        name: String,
        root_prop: Prop,
        configs: HashMap<Id, Config>,
        timestamps: Timestamps,
        version: Version,
        event_collector: Option<EventCollector>,
    ) -> Result<Schema, Error> {
        if name.is_empty() {
            return Err(Error::EmptyName);
        }

        Ok(Schema {
            id,
            name,
            root_prop,
            configs,
            timestamps,
            version,
            event_collector: event_collector.unwrap_or_else(EventCollector::create),
        })
    }

    pub fn create(id: Id, name: String, root_prop: Prop) -> Result<Schema, Error> {
        let mut schema = Schema::new(
            id,
            name,
            root_prop,
            HashMap::new(),
            Timestamps::create(),
            Version::init_version(),
            Some(EventCollector::create()),
        )?;

        schema
            .event_collector
            .record(SchemaCreated {
                id: schema.id().to_string(),
                name: schema.name().to_string(),
                root_prop: schema.root_prop().clone().try_into()?,
            })
            .map_err(Error::CouldNotRecordEvent)?;

        Ok(schema)
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn root_prop(&self) -> &Prop {
        &self.root_prop
    }

    pub fn configs(&self) -> &HashMap<Id, Config> {
        &self.configs
    }

    pub fn timestamps(&self) -> &Timestamps {
        &self.timestamps
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn events(&mut self) -> Vec<Event> {
        self.event_collector.drain()
    }

    pub fn change_root_prop(&mut self, prop: Prop) -> Result<(), Error> {
        self.root_prop = prop;

        for config in self.configs.values_mut() {
            let diff = self.root_prop.validate(config.data());
            if !diff.is_empty() {
                config.mark_as_invalid();
            }
        }

        self.event_collector
            .record(SchemaRootPropChanged {
                id: self.id.to_string(),
                root_prop: self.root_prop.clone().try_into()?,
            })
            .map_err(Error::CouldNotRecordEvent)?;

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();

        Ok(())
    }

    pub fn get_config(&mut self, id: &Id) -> Result<&Config, Error> {
        if let Some(config) = self.configs.get(id) {
            self.event_collector
                .record(ConfigAccessed {
                    id: config.id().to_string(),
                    schema_id: self.id.to_string(),
                })
                .map_err(Error::CouldNotRecordEvent)?;

            Ok(config)
        } else {
            Err(Error::ConfigNotFound(id.clone()))
        }
    }

    pub fn add_config(
        &mut self,
        id: Id,
        name: String,
        data: Value,
        checksum: Vec<u8>,
    ) -> Result<(), Error> {
        if self.configs.contains_key(&id) {
            return Err(Error::ConfigAlreadyExists(id));
        }

        let diff = self.root_prop.validate(&data);
        if !diff.is_empty() {
            return Err(Error::InvalidConfig(diff));
        }

        let config = Config::create(id, name, data, diff.is_empty(), checksum)?;

        self.event_collector
            .record(ConfigCreated {
                id: config.id().to_string(),
                schema_id: self.id.to_string(),
                data: config.data().into(),
                valid: config.is_valid(),
            })
            .map_err(Error::CouldNotRecordEvent)?;

        self.configs.insert(config.id().clone(), config);

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();

        Ok(())
    }

    pub fn update_config(&mut self, id: &Id, data: Value, checksum: Vec<u8>) -> Result<(), Error> {
        if let Some(config) = self.configs.get_mut(id) {
            let diff = self.root_prop.validate(&data);
            if !diff.is_empty() {
                return Err(Error::InvalidConfig(diff));
            }

            config.change_data(data, diff.is_empty(), checksum)?;

            self.event_collector
                .record(ConfigDataChanged {
                    id: config.id().to_string(),
                    schema_id: self.id.to_string(),
                    data: config.data().into(),
                    valid: config.is_valid(),
                })
                .map_err(Error::CouldNotRecordEvent)?;

            self.timestamps = self.timestamps.update();
            self.version = self.version.incr();

            return Ok(());
        }

        Err(Error::ConfigNotFound(id.clone()))
    }

    pub fn delete_config(&mut self, id: &Id) -> Result<(), Error> {
        if !self.configs.contains_key(id) {
            return Err(Error::ConfigNotFound(id.clone()));
        }

        self.configs.remove(id);

        self.event_collector
            .record(ConfigDeleted {
                id: id.to_string(),
                schema_id: self.id.to_string(),
            })
            .map_err(Error::CouldNotRecordEvent)?;

        Ok(())
    }

    pub fn delete(&mut self) -> Result<(), Error> {
        if !self.configs.is_empty() {
            return Err(Error::SchemaContainsConfigs(self.id.clone()));
        }

        self.event_collector
            .record(SchemaDeleted {
                id: self.id.to_string(),
            })
            .map_err(Error::CouldNotRecordEvent)?;

        self.timestamps = self.timestamps.delete();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use crate::domain::{Interval, Value};

    #[test]
    fn create() {
        let schema = Schema::create(
            Id::new("schema-01").unwrap(),
            "Schema 01".to_string(),
            Prop::bool(true, None).unwrap(),
        )
        .unwrap();

        assert_eq!(schema.id().value(), "schema-01");
        assert_eq!(schema.name(), "Schema 01");
        assert_eq!(schema.root_prop(), &Prop::bool(true, None).unwrap());
    }

    #[test]
    fn validate() {
        let schema = Schema::new(
            Id::new("schema#01").unwrap(),
            "Schema 01".to_string(),
            Prop::object(BTreeMap::from([
                (
                    "env".to_string(),
                    Prop::string(
                        true,
                        Some(Value::String("dev".to_string())),
                        Some(vec![
                            Value::String("dev".to_string()),
                            Value::String("stg".to_string()),
                            Value::String("prod".to_string()),
                        ]),
                        None,
                    )
                    .unwrap(),
                ),
                (
                    "num".to_string(),
                    Prop::int(true, None, None, Some(Interval::new(1, 5).unwrap())).unwrap(),
                ),
            ])),
            HashMap::new(),
            Timestamps::create(),
            Version::init_version(),
            None,
        )
        .unwrap();

        assert!(schema
            .root_prop()
            .validate(&Value::Object(BTreeMap::from([
                ("env".to_string(), Value::String("stg".to_string())),
                ("num".to_string(), Value::Int(4)),
            ])))
            .is_empty());

        assert!(schema
            .root_prop()
            .validate(&Value::Object(BTreeMap::from([
                ("env".to_string(), Value::String("stg".to_string())),
                ("num".to_string(), Value::Int(4)),
            ])))
            .is_empty());
        assert!(!schema
            .root_prop()
            .validate(&Value::Object(BTreeMap::from([
                ("env".to_string(), Value::String("other".to_string())),
                ("num".to_string(), Value::Int(4)),
            ])))
            .is_empty());
        assert!(!schema
            .root_prop()
            .validate(&Value::Object(BTreeMap::from([
                ("env".to_string(), Value::String("stg".to_string())),
                ("num".to_string(), Value::Int(9)),
            ])))
            .is_empty());
        assert!(!schema
            .root_prop()
            .validate(&Value::Object(BTreeMap::from([(
                "env".to_string(),
                Value::String("stg".to_string())
            )])))
            .is_empty());
        assert!(!schema
            .root_prop()
            .validate(&Value::Object(BTreeMap::from([
                ("env".to_string(), Value::String("stg".to_string())),
                ("num".to_string(), Value::Int(4)),
                ("non_existing".to_string(), Value::Int(1)),
            ])))
            .is_empty());
    }
}
