use async_trait::async_trait;
use std::collections::HashMap;

use crate::domain::{Error, Prop, Value};

#[async_trait]
pub trait SchemaRepository {
    async fn find_by_id(&self, id: &SchemaId) -> Result<Option<Schema>, Error>;
    async fn save(&self, schema: &mut Schema) -> Result<(), Error>;
    async fn delete(&self, id: &SchemaId) -> Result<(), Error>;
}

pub struct SchemaId {
    id: String,
}

impl SchemaId {
    pub fn new<I: Into<String>>(id: I) -> Result<SchemaId, Error> {
        let id = id.into();

        if id.is_empty() {
            return Err(Error::Generic);
        }

        Ok(SchemaId { id })
    }

    pub fn value(&self) -> &str {
        &self.id
    }
}

impl ToString for SchemaId {
    fn to_string(&self) -> String {
        self.id.to_string()
    }
}

pub struct Schema {
    id: SchemaId,
    name: String,

    props: HashMap<String, Prop>,
}

impl Schema {
    pub fn new<N: Into<String>>(
        id: SchemaId,
        name: N,
        props: HashMap<String, Prop>,
    ) -> Result<Schema, Error> {
        let name = name.into();

        if name.is_empty() {
            return Err(Error::Generic);
        }

        Ok(Schema { id, name, props })
    }

    pub fn create<N: Into<String>>(id: SchemaId, name: N) -> Result<Schema, Error> {
        Schema::new(id, name, HashMap::new())
    }

    pub fn id(&self) -> &SchemaId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn props(&self) -> &HashMap<String, Prop> {
        &self.props
    }

    pub fn add_prop<K: Into<String>>(&mut self, key: K, prop: Prop) -> Result<(), Error> {
        self.props.insert(key.into(), prop);

        Ok(())
    }

    pub fn validate(&self, config: &HashMap<String, Value>) -> Result<(), Error> {
        for (key, value) in config.iter() {
            if let Some(prop) = self.props.get(key) {
                if let Err(err) = prop.validate_value(value) {
                    return Err(err);
                }
            } else {
                return Err(Error::Generic);
            }
        }

        for (key, value) in self.props.iter() {
            if value.is_required() && config.get(key).is_none() {
                return Err(Error::Generic);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::{Interval, Kind};

    #[test]
    fn create() {
        let schema = Schema::create(SchemaId::new("schema#01").unwrap(), "Schema 01").unwrap();

        assert_eq!(schema.id().value(), "schema#01");
        assert_eq!(schema.name(), "Schema 01");
        assert!(schema.props().is_empty());
    }

    #[test]
    fn validate() {
        let schema = Schema::new(
            SchemaId::new("schema#01").unwrap(),
            "Schema 01",
            HashMap::from([
                (
                    "env".to_string(),
                    Prop::new(
                        "env",
                        Kind::String,
                        false,
                        Some(Value::String("dev".to_string())),
                        true,
                        Some(vec![
                            Value::String("dev".to_string()),
                            Value::String("stg".to_string()),
                            Value::String("prod".to_string()),
                        ]),
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                ),
                (
                    "num".to_string(),
                    Prop::new(
                        "num",
                        Kind::Int,
                        false,
                        None,
                        true,
                        None,
                        None,
                        Some(Interval::new(2_f64, 8_f64).unwrap()),
                        None,
                    )
                    .unwrap(),
                ),
            ]),
        )
        .unwrap();

        assert!(schema
            .validate(&HashMap::from([
                ("env".to_string(), Value::String("stg".to_string())),
                ("num".to_string(), Value::Int(4)),
            ]))
            .is_ok());
        assert!(schema
            .validate(&HashMap::from([
                ("env".to_string(), Value::String("other".to_string())),
                ("num".to_string(), Value::Int(4)),
            ]))
            .is_err());
        assert!(schema
            .validate(&HashMap::from([
                ("env".to_string(), Value::String("stg".to_string())),
                ("num".to_string(), Value::Int(9)),
            ]))
            .is_err());
        assert!(schema
            .validate(&HashMap::from([(
                "env".to_string(),
                Value::String("stg".to_string())
            ),]))
            .is_err());
        assert!(schema
            .validate(&HashMap::from([
                ("env".to_string(), Value::String("stg".to_string())),
                ("num".to_string(), Value::Int(4)),
                ("non_existing".to_string(), Value::Int(1)),
            ]))
            .is_err());
    }
}
