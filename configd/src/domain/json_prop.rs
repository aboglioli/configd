use serde::{Deserialize, Serialize};
use serde_json::{Map, Value as JsonValue};
use std::collections::BTreeMap;

use crate::domain::{Error, Interval, Prop, Value};

const SCHEMA_KEY: &str = "$schema";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum JsonPropKind {
    Bool,
    Int,
    Float,
    String,
}

#[derive(Serialize, Deserialize)]
struct JsonPropInterval {
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,
}

#[derive(Serialize, Deserialize)]
struct JsonProp {
    kind: JsonPropKind,
    required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_values: Option<Vec<JsonValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<JsonPropInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<String>,
}

impl TryFrom<JsonValue> for Prop {
    type Error = Error;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Object(mut map) => {
                // $schema
                if let Some(value) = map.remove(SCHEMA_KEY) {
                    let prop: JsonProp =
                        serde_json::from_value(value).map_err(Error::CouldNotDeserializeProp)?;

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
                    object.insert(key, Self::try_from(value)?);
                }

                Ok(Prop::object(object))
            }
            JsonValue::Array(mut items) => {
                if items.len() != 1 {
                    return Err(Error::InvalidArray);
                }

                Ok(Prop::array(Self::try_from(items.remove(0))?))
            }
            _ => Err(Error::UnknownRootProp),
        }
    }
}

impl TryFrom<Prop> for JsonValue {
    type Error = Error;

    fn try_from(prop: Prop) -> Result<Self, Self::Error> {
        let value = match prop {
            Prop::Bool {
                required,
                default_value,
            } => {
                let json_prop = JsonProp {
                    kind: JsonPropKind::Bool,
                    required,
                    default_value: default_value.map(JsonValue::from),
                    allowed_values: None,
                    interval: None,
                    regex: None,
                };

                let json_value =
                    serde_json::to_value(&json_prop).map_err(Error::CouldNotDeserializeProp)?;

                let mut map = Map::new();
                map.insert(SCHEMA_KEY.to_string(), json_value);

                JsonValue::Object(map)
            }
            Prop::Int {
                required,
                default_value,
                allowed_values,
                interval,
            } => {
                let json_prop = JsonProp {
                    kind: JsonPropKind::Int,
                    required,
                    default_value: default_value.map(JsonValue::from),
                    allowed_values: allowed_values
                        .map(|values| values.into_iter().map(JsonValue::from).collect()),
                    interval: interval.map(|interval| JsonPropInterval {
                        min: interval.min(),
                        max: interval.max(),
                    }),
                    regex: None,
                };

                let json_value =
                    serde_json::to_value(&json_prop).map_err(Error::CouldNotDeserializeProp)?;

                let mut map = Map::new();
                map.insert(SCHEMA_KEY.to_string(), json_value);

                JsonValue::Object(map)
            }
            Prop::Float {
                required,
                default_value,
                allowed_values,
                interval,
            } => {
                let json_prop = JsonProp {
                    kind: JsonPropKind::Float,
                    required,
                    default_value: default_value.map(JsonValue::from),
                    allowed_values: allowed_values
                        .map(|values| values.into_iter().map(JsonValue::from).collect()),
                    interval: interval.map(|interval| JsonPropInterval {
                        min: interval.min(),
                        max: interval.max(),
                    }),
                    regex: None,
                };

                let json_value =
                    serde_json::to_value(&json_prop).map_err(Error::CouldNotDeserializeProp)?;

                let mut map = Map::new();
                map.insert(SCHEMA_KEY.to_string(), json_value);

                JsonValue::Object(map)
            }
            Prop::String {
                required,
                default_value,
                allowed_values,
                regex,
            } => {
                let json_prop = JsonProp {
                    kind: JsonPropKind::String,
                    required,
                    default_value: default_value.map(JsonValue::from),
                    allowed_values: allowed_values
                        .map(|values| values.into_iter().map(JsonValue::from).collect()),
                    interval: None,
                    regex,
                };

                let json_value =
                    serde_json::to_value(&json_prop).map_err(Error::CouldNotDeserializeProp)?;

                let mut map = Map::new();
                map.insert(SCHEMA_KEY.to_string(), json_value);

                JsonValue::Object(map)
            }
            Prop::Array(prop) => JsonValue::Array(vec![Self::try_from(*prop)?]),
            Prop::Object(map) => {
                let mut object = Map::new();

                for (key, prop) in map.into_iter() {
                    object.insert(key, Self::try_from(prop)?);
                }

                JsonValue::Object(object)
            }
        };

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_from_json_string() {
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
        let value: JsonValue = serde_json::from_str(&json).unwrap();
        let prop: Prop = value.try_into().unwrap();

        assert_eq!(
            prop,
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

    #[test]
    fn convert_to_json_string() {
        let prop = Prop::Object(BTreeMap::from([
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
                    None,
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
                ),
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
                    ),
                ])),
            ),
            (
                "extra_services".to_string(),
                Prop::array(Prop::object(BTreeMap::from([
                    ("id".to_string(), Prop::int(true, None, None, None).unwrap()),
                    (
                        "name".to_string(),
                        Prop::string(false, None, None, None).unwrap(),
                    ),
                ]))),
            ),
        ]));

        let json_value: JsonValue = prop.try_into().unwrap();

        assert_eq!(
            json_value,
            serde_json::from_str::<JsonValue>(
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
                            "min": 2.0,
                            "max": 10.0
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
                                "min": 1024.0
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
            }"#,
            )
            .unwrap(),
        );
    }
}
