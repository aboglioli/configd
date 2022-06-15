use async_trait::async_trait;

use crate::domain::{Error, Prop, Value};

pub trait SchemaBuilder<P> {
    type Error;

    fn build(&self, id: SchemaId, name: String, props: P) -> Result<Schema, Self::Error>;
}

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

    root_prop: Option<Prop>,
}

impl Schema {
    pub fn new<N: Into<String>>(
        id: SchemaId,
        name: N,
        root_prop: Option<Prop>,
    ) -> Result<Schema, Error> {
        let name = name.into();

        if name.is_empty() {
            return Err(Error::Generic);
        }

        Ok(Schema {
            id,
            name,
            root_prop,
        })
    }

    pub fn create<N: Into<String>>(id: SchemaId, name: N) -> Result<Schema, Error> {
        Schema::new(id, name, None)
    }

    pub fn id(&self) -> &SchemaId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn root_prop(&self) -> Option<&Prop> {
        self.root_prop.as_ref()
    }

    pub fn change_root_prop<K: Into<String>>(&mut self, prop: Prop) -> Result<(), Error> {
        self.root_prop = Some(prop);
        Ok(())
    }

    pub fn validate(&self, value: &Value) -> bool {
        if let Some(prop) = &self.root_prop {
            prop.validate(value)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use crate::domain::Interval;

    #[test]
    fn create() {
        let schema = Schema::create(SchemaId::new("schema#01").unwrap(), "Schema 01").unwrap();

        assert_eq!(schema.id().value(), "schema#01");
        assert_eq!(schema.name(), "Schema 01");
        assert!(schema.root_prop().is_none());
    }

    #[test]
    fn validate() {
        let schema = Schema::new(
            SchemaId::new("schema#01").unwrap(),
            "Schema 01",
            Some(Prop::object(BTreeMap::from([
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
            ]))),
        )
        .unwrap();

        assert!(schema.validate(&Value::Object(BTreeMap::from([
            ("env".to_string(), Value::String("stg".to_string())),
            ("num".to_string(), Value::Int(4)),
        ]))));

        assert!(schema.validate(&Value::Object(BTreeMap::from([
            ("env".to_string(), Value::String("stg".to_string())),
            ("num".to_string(), Value::Int(4)),
        ]))));
        assert!(!schema.validate(&Value::Object(BTreeMap::from([
            ("env".to_string(), Value::String("other".to_string())),
            ("num".to_string(), Value::Int(4)),
        ]))));
        assert!(!schema.validate(&Value::Object(BTreeMap::from([
            ("env".to_string(), Value::String("stg".to_string())),
            ("num".to_string(), Value::Int(9)),
        ]),)));
        assert!(!schema.validate(&Value::Object(BTreeMap::from([(
            "env".to_string(),
            Value::String("stg".to_string())
        )]))));
        assert!(!schema.validate(&Value::Object(BTreeMap::from([
            ("env".to_string(), Value::String("stg".to_string())),
            ("num".to_string(), Value::Int(4)),
            ("non_existing".to_string(), Value::Int(1)),
        ]))));
    }
}
