use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

use crate::domain::{Error, Interval, Prop, PropConverter, Value};

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
    kind: JsonPropKind,
    required: bool,
    default_value: Option<JsonValue>,
    allowed_values: Option<Vec<JsonValue>>,
    interval: Option<JsonPropInterval>,
    regex: Option<String>,
}

pub struct JsonPropConverter;

impl JsonPropConverter {
    const SCHEMA_KEY: &'static str = "$schema";

    pub fn new() -> JsonPropConverter {
        JsonPropConverter
    }
}

impl PropConverter<String> for JsonPropConverter {
    type Error = Error;

    fn from(&self, props: String) -> Result<Prop, Self::Error> {
        let value: JsonValue = serde_json::from_str(&props).map_err(|_| Error::Generic)?;

        self.from(value)
    }

    fn to(&self, prop: &Prop) -> Result<String, Self::Error> {
        let value: JsonValue = self.to(prop)?;

        serde_json::to_string(&value).map_err(|_| Error::Generic)
    }
}

impl PropConverter<JsonValue> for JsonPropConverter {
    type Error = Error;

    fn from(&self, props: JsonValue) -> Result<Prop, Self::Error> {
        match props {
            JsonValue::Object(mut map) => {
                // $schema
                if let Some(value) = map.remove(Self::SCHEMA_KEY) {
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

                    return match prop.kind {
                        JsonPropKind::Bool => Prop::bool(prop.required, default_value),
                        JsonPropKind::Int => {
                            Prop::int(prop.required, default_value, allowed_values, interval)
                        }
                        JsonPropKind::Float => {
                            Prop::float(prop.required, default_value, allowed_values, interval)
                        }
                        JsonPropKind::String => {
                            Prop::string(prop.required, default_value, allowed_values, prop.regex)
                        }
                    };
                }

                // Object
                let mut object = BTreeMap::new();
                for (key, value) in map.into_iter() {
                    object.insert(key, self.from(value)?);
                }

                return Ok(Prop::object(object));
            }
            JsonValue::Array(mut items) => {
                if items.len() != 1 {
                    return Err(Error::Generic);
                }

                Ok(Prop::array(self.from(items.remove(0))?))
            }
            _ => Err(Error::Generic),
        }
    }

    fn to(&self, prop: &Prop) -> Result<JsonValue, Error> {
        Err(Error::Generic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_from_json_string() {
        let builder = JsonPropConverter::new();
        let json = r#"{
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
            ],
            "custom_service": {
                "urls": [
                    {
                        "$schema": {
                            "kind": "string",
                            "required": true,
                            "default_value": "http://localhost",
                            "regex": "^http://[a-z]+0[0-9]{1}$"
                        }
                    }
                ],
                "port": {
                    "$schema": {
                        "kind": "int",
                        "required": false,
                        "default_value": 1234,
                        "interval": {
                            "min": 1024
                        }
                    }
                }
            },
            "extra_services": [
                {
                    "id": {
                        "$schema": {
                            "kind": "int",
                            "required": true
                        }
                    },
                    "name": {
                        "$schema": {
                            "kind": "string",
                            "required": false
                        }
                    }
                }
            ]
        }"#
        .to_string();

        assert_eq!(
            builder.from(json).unwrap(),
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
                ),
                (
                    "custom_service".to_string(),
                    Prop::object(BTreeMap::from([
                        (
                            "urls".to_string(),
                            Prop::array(
                                Prop::string(
                                    true,
                                    Some(Value::String("http://localhost".to_string())),
                                    None,
                                    Some("^http://[a-z]+0[0-9]{1}$".to_string()),
                                )
                                .unwrap(),
                            ),
                        ),
                        (
                            "port".to_string(),
                            Prop::int(
                                false,
                                Some(Value::Int(1234)),
                                None,
                                Some(Interval::new(1024, None).unwrap()),
                            )
                            .unwrap(),
                        )
                    ]))
                ),
                (
                    "extra_services".to_string(),
                    Prop::array(Prop::object(BTreeMap::from([
                        ("id".to_string(), Prop::int(true, None, None, None).unwrap()),
                        (
                            "name".to_string(),
                            Prop::string(false, None, None, None).unwrap(),
                        ),
                    ])),),
                )
            ])),
        );
    }
}
