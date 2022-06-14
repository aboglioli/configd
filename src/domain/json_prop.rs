use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

use crate::domain::{Error, Interval, Prop, PropBuilder, Value};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum JsonPropKind {
    Bool,
    Int,
    Float,
    String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct JsonPropInterval {
    min: Option<f64>,
    max: Option<f64>,
}

#[derive(Serialize, Deserialize)]
struct JsonProp {
    kind: String,
    required: bool,
    default_value: Option<JsonValue>,
    allowed_values: Option<Vec<JsonValue>>,
    interval: Option<JsonPropInterval>,
    regex: Option<String>,
}

pub struct JsonPropBuilder;

impl JsonPropBuilder {
    const SCHEMA_KEY: &'static str = "$schema";

    pub fn new() -> JsonPropBuilder {
        JsonPropBuilder
    }
}

impl PropBuilder<&str> for JsonPropBuilder {
    type Error = Error;

    fn build(&self, props: &str) -> Result<Prop, Self::Error> {
        let value: JsonValue = serde_json::from_str(props).map_err(|_| Error::Generic)?;

        self.build(value)
    }
}

impl PropBuilder<JsonValue> for JsonPropBuilder {
    type Error = Error;

    fn build(&self, props: JsonValue) -> Result<Prop, Self::Error> {
        match props {
            JsonValue::Object(map) => {
                let mut object = BTreeMap::new();

                for (key, value) in map.into_iter() {
                    if key == Self::SCHEMA_KEY {
                        let prop: JsonProp =
                            serde_json::from_value(value).map_err(|_| Error::Generic)?;

                        let default_value = prop.default_value.map(Value::from);
                        let allowed_values = prop
                            .allowed_values
                            .map(|values| values.into_iter().map(Value::from).collect());
                        let interval = prop
                            .interval
                            .map(|interval| Interval::new::<f64, _, _>(interval.min, interval.max))
                            .transpose()?;

                        return match prop.kind.to_lowercase().as_str() {
                            "bool" => Prop::bool(prop.required, default_value),
                            "int" => {
                                Prop::int(prop.required, default_value, allowed_values, interval)
                            }
                            "float" => {
                                Prop::float(prop.required, default_value, allowed_values, interval)
                            }
                            "string" => Prop::string(
                                prop.required,
                                default_value,
                                allowed_values,
                                prop.regex,
                            ),
                            _ => Err(Error::Generic),
                        };
                    }

                    object.insert(key, self.build(value)?);
                }

                return Ok(Prop::object(object));
            }
            JsonValue::Array(mut items) => {
                if items.len() != 1 {
                    return Err(Error::Generic);
                }

                Ok(Prop::array(self.build(items.remove(0))?))
            }
            _ => Err(Error::Generic),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn build_from_json_string() {
        let builder = JsonPropBuilder::new();

        assert_eq!(
            builder
                .build(
                    r#"{
                    "env": {
                        "$schema": {
                            "kind": "string",
                            "required": true,
                            "allowed_values": ["dev", "stg", "prod"]
                        }
                    },
                    "instances": {
                        "$schema": {
                            "kind": "int",
                            "required": false,
                            "default_value": 3,
                            "interval": {
                                "min": 2,
                                "max": 10
                            }
                        }
                    },
                    "database_urls": [
                        {
                            "$schema": {
                                "kind": "string",
                                "required": true,
                                "default_value": "http://localhost:1234",
                                "regex": "^http://[a-z]+:[0-9]{2,4}$"
                            }
                        }
                    ]
                }"#,
                )
                .unwrap(),
            Prop::Object(BTreeMap::from([
                (
                    "env".to_string(),
                    Prop::string(
                        true,
                        None,
                        Some(vec![
                            Value::String("dev".to_string()),
                            Value::String("stg".to_string()),
                            Value::String("prod".to_string()),
                        ]),
                        None
                    )
                    .unwrap(),
                ),
                (
                    "instances".to_string(),
                    Prop::int(
                        false,
                        Some(Value::Int(3)),
                        None,
                        Some(Interval::new(2, 10).unwrap()),
                    )
                    .unwrap(),
                ),
                (
                    "database_urls".to_string(),
                    Prop::array(
                        Prop::string(
                            true,
                            Some(Value::String("http://localhost:1234".to_string())),
                            None,
                            Some("^http://[a-z]+:[0-9]{2,4}$".to_string()),
                        )
                        .unwrap(),
                    )
                )
            ])),
        );
    }
}
