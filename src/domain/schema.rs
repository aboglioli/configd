use async_trait::async_trait;

use crate::domain::{Error, Prop, SchemaId, Value};

#[async_trait]
pub trait SchemaRepository {
    async fn find_by_id(&self, id: &SchemaId) -> Result<Option<Schema>, Error>;
    async fn save(&self, schema: &mut Schema) -> Result<(), Error>;
    async fn delete(&self, id: &SchemaId) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct Schema {
    id: SchemaId,
    name: String,

    root_prop: Prop,
}

impl Schema {
    pub fn new(id: SchemaId, name: String, root_prop: Prop) -> Result<Schema, Error> {
        if name.is_empty() {
            return Err(Error::Generic);
        }

        Ok(Schema {
            id,
            name,
            root_prop,
        })
    }

    pub fn create(name: String, root_prop: Prop) -> Result<Schema, Error> {
        Schema::new(SchemaId::slug(&name)?, name, root_prop)
    }

    pub fn id(&self) -> &SchemaId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn root_prop(&self) -> &Prop {
        &self.root_prop
    }

    pub fn into_root_prop(self) -> Prop {
        self.root_prop
    }

    pub fn change_root_prop(&mut self, prop: Prop) -> Result<(), Error> {
        self.root_prop = prop;
        Ok(())
    }

    pub fn validate(&self, value: &Value) -> bool {
        self.root_prop.validate(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use crate::domain::Interval;

    #[test]
    fn create() {
        let schema =
            Schema::create("Schema 01".to_string(), Prop::bool(true, None).unwrap()).unwrap();

        assert_eq!(schema.id().value(), "schema-01");
        assert_eq!(schema.name(), "Schema 01");
        assert_eq!(schema.root_prop(), &Prop::bool(true, None).unwrap());
    }

    #[test]
    fn validate() {
        let schema = Schema::new(
            SchemaId::new("schema#01").unwrap(),
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
