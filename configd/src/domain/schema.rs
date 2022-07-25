use async_trait::async_trait;
use core_lib::{
    events::{Event, EventCollector},
    models::{Timestamps, Version},
};
use std::collections::HashMap;

use crate::domain::{
    Access, Config, ConfigAccessRemoved, ConfigAccessed, ConfigCreated, ConfigDataChanged,
    ConfigDeleted, Error, Id, Page, Password, Prop, SchemaCreated, SchemaDeleted,
    SchemaRootPropChanged, Value,
};

#[async_trait]
pub trait SchemaRepository {
    async fn find(&self, offset: Option<u64>, limit: Option<u64>) -> Result<Page<Schema>, Error>;
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
            .map_err(Error::Core)?;

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

    // Mutations
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
            .map_err(Error::Core)?;

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();

        Ok(())
    }

    pub fn get_config(
        &mut self,
        id: &Id,
        access: Access,
        password: Option<&Password>,
    ) -> Result<Config, Error> {
        let config = self
            .configs
            .get_mut(id)
            .ok_or_else(|| Error::ConfigNotFound(id.clone()))?;

        if !config.can_access(password) {
            return Err(Error::Unauthorized);
        }

        self.event_collector
            .record(ConfigAccessed {
                id: config.id().to_string(),
                schema_id: self.id.to_string(),
                source: access.source().to_string(),
                instance: access.instance().to_string(),
            })
            .map_err(Error::Core)?;

        config.register_access(access);

        Ok(config.clone())
    }

    pub fn populate_config(&self, config: &Config) -> Value {
        self.root_prop
            .populate(config.data(), config.accesses().len() as i64)
    }

    pub fn add_config(
        &mut self,
        id: Id,
        name: String,
        data: Value,
        password: Option<Password>,
    ) -> Result<(), Error> {
        if self.configs.contains_key(&id) {
            return Err(Error::ConfigAlreadyExists(id));
        }

        let diff = self.root_prop.validate(&data);
        if !diff.is_empty() {
            return Err(Error::InvalidConfig(diff));
        }

        let config = Config::create(id, name, data, diff.is_empty(), password)?;

        self.event_collector
            .record(ConfigCreated {
                id: config.id().to_string(),
                schema_id: self.id.to_string(),
                name: config.name().to_string(),
                data: config.data().into(),
                valid: config.is_valid(),
            })
            .map_err(Error::Core)?;

        self.configs.insert(config.id().clone(), config);

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();

        Ok(())
    }

    pub fn update_config(
        &mut self,
        id: &Id,
        data: Value,
        password: Option<&Password>,
    ) -> Result<(), Error> {
        let config = self
            .configs
            .get_mut(id)
            .ok_or_else(|| Error::ConfigNotFound(id.clone()))?;

        if !config.can_access(password) {
            return Err(Error::Unauthorized);
        }

        let diff = self.root_prop.validate(&data);
        if !diff.is_empty() {
            return Err(Error::InvalidConfig(diff));
        }

        config.change_data(data, diff.is_empty())?;

        self.event_collector
            .record(ConfigDataChanged {
                id: config.id().to_string(),
                schema_id: self.id.to_string(),
                data: config.data().into(),
                valid: config.is_valid(),
            })
            .map_err(Error::Core)?;

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();

        Ok(())
    }

    pub fn change_config_password(
        &mut self,
        id: &Id,
        old_password: Option<&Password>,
        new_password: Password,
    ) -> Result<(), Error> {
        let config = self
            .configs
            .get_mut(id)
            .ok_or_else(|| Error::ConfigNotFound(id.clone()))?;

        config.change_password(old_password, new_password)?;

        Ok(())
    }

    pub fn delete_config_password(
        &mut self,
        id: &Id,
        password: Option<&Password>,
    ) -> Result<(), Error> {
        let config = self
            .configs
            .get_mut(id)
            .ok_or_else(|| Error::ConfigNotFound(id.clone()))?;

        config.delete_password(password)?;

        Ok(())
    }

    pub fn clean_config_accesses(&mut self, id: &Id) -> Result<(), Error> {
        let config = self
            .configs
            .get_mut(id)
            .ok_or_else(|| Error::ConfigNotFound(id.clone()))?;

        let removed_accesses = config.clean_old_accesses();

        for access in removed_accesses.into_iter() {
            self.event_collector
                .record(ConfigAccessRemoved {
                    id: config.id().to_string(),
                    schema_id: self.id.to_string(),
                    source: access.source().to_string(),
                    instance: access.instance().to_string(),
                })
                .map_err(Error::Core)?;
        }

        Ok(())
    }

    pub fn delete_config(&mut self, id: &Id, password: Option<&Password>) -> Result<(), Error> {
        let config = self
            .configs
            .get_mut(id)
            .ok_or_else(|| Error::ConfigNotFound(id.clone()))?;

        if !config.can_access(password) {
            return Err(Error::Unauthorized);
        }

        self.configs.remove(id);

        self.event_collector
            .record(ConfigDeleted {
                id: id.to_string(),
                schema_id: self.id.to_string(),
            })
            .map_err(Error::Core)?;

        self.timestamps = self.timestamps.update();
        self.version = self.version.incr();

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
            .map_err(Error::Core)?;

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
        let schema = Schema::create(
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
                    Prop::int(true, None, None, Some(Interval::new(1, 5).unwrap()), false).unwrap(),
                ),
            ])),
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

    #[test]
    fn add_and_get_populated_config() {
        let mut schema = Schema::create(
            Id::new("schema-01").unwrap(),
            "Schema 01".to_string(),
            Prop::string(true, Some(Value::String("default".to_string())), None, None).unwrap(),
        )
        .unwrap();

        // Create config
        let config_id = Id::new("config-01").unwrap();

        schema
            .add_config(
                config_id.clone(),
                "Config 01".to_string(),
                Value::Null,
                None,
            )
            .unwrap();

        // Get config
        let config = schema
            .get_config(&config_id, Access::unknown(), None)
            .unwrap();

        assert_eq!(config.id(), &config_id);
        assert_eq!(config.name(), "Config 01");
        assert_eq!(config.data(), &Value::Null);

        // Populate data
        let data = schema.populate_config(&config);
        assert_eq!(data, Value::String("default".to_string()));
    }
}
