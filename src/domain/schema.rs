use async_trait::async_trait;
use std::collections::HashMap;

use crate::domain::{Config, Error, Id, Prop, Value};

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
}

impl Schema {
    pub fn new(
        id: Id,
        name: String,
        root_prop: Prop,
        configs: HashMap<Id, Config>,
    ) -> Result<Schema, Error> {
        if name.is_empty() {
            return Err(Error::Generic);
        }

        Ok(Schema {
            id,
            name,
            root_prop,
            configs,
        })
    }

    pub fn create(id: Id, name: String, root_prop: Prop) -> Result<Schema, Error> {
        Schema::new(id, name, root_prop, HashMap::new())
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

    pub fn change_root_prop(&mut self, prop: Prop) -> Result<(), Error> {
        self.root_prop = prop;

        Ok(())
    }

    pub fn get_config(&self, id: &Id) -> Option<&Config> {
        self.configs.get(id)
    }

    pub fn add_config(&mut self, id: Id, name: String, data: Value) -> Result<(), Error> {
        if self.configs.contains_key(&id) {
            return Err(Error::Generic);
        }

        let diff = self.root_prop.validate(&data);

        let config = Config::create(id, name, data, diff.is_empty())?;
        self.configs.insert(config.id().clone(), config);

        Ok(())
    }

    pub fn update_config(&mut self, id: &Id, data: Value) -> Result<(), Error> {
        if let Some(config) = self.configs.get_mut(id) {
            let diff = self.root_prop.validate(&data);

            config.change_data(data)?;
            config.set_valid(diff.is_empty());

            return Ok(());
        }

        Err(Error::Generic)
    }

    pub fn delete_config(&mut self, id: &Id) -> Result<(), Error> {
        if !self.configs.contains_key(id) {
            return Err(Error::Generic);
        }

        self.configs.remove(id);

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
