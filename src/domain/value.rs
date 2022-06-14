use serde_json::Value as JsonValue;
use std::collections::{BTreeMap, HashMap};

// Value & Kind
#[derive(Debug, PartialEq)]
pub enum Kind {
    Null,
    Bool,
    Int,
    Float,
    String,
    Array,
    Object,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl Value {
    pub fn kind(&self) -> Kind {
        match self {
            Value::Null => Kind::Null,
            Value::Bool(_) => Kind::Bool,
            Value::Int(_) => Kind::Int,
            Value::Float(_) => Kind::Float,
            Value::String(_) => Kind::String,
            Value::Array(_) => Kind::Array,
            Value::Object(_) => Kind::Object,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(values: Vec<T>) -> Self {
        Value::Array(values.into_iter().map(|value| value.into()).collect())
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Value::Null,
        }
    }
}

impl<K, V> From<BTreeMap<K, V>> for Value
where
    K: Into<String>,
    V: Into<Value>,
{
    fn from(values: BTreeMap<K, V>) -> Self {
        Value::Object(
            values
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<K, V> From<HashMap<K, V>> for Value
where
    K: Into<String>,
    V: Into<Value>,
{
    fn from(values: HashMap<K, V>) -> Self {
        Value::Object(
            values
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl From<JsonValue> for Value {
    fn from(value: JsonValue) -> Self {
        match value {
            JsonValue::Null => Value::Null,
            JsonValue::Bool(value) => Value::Bool(value),
            JsonValue::Number(value) => {
                if let Some(value) = value.as_i64() {
                    Value::Int(value)
                } else if let Some(value) = value.as_f64() {
                    Value::Float(value)
                } else {
                    Value::Null
                }
            }
            JsonValue::String(value) => Value::String(value),
            JsonValue::Array(values) => Value::Array(values.into_iter().map(Value::from).collect()),
            JsonValue::Object(values) => Value::Object(
                values
                    .into_iter()
                    .map(|(key, value)| (key, Value::from(value)))
                    .collect(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        assert_eq!(Value::from(true), Value::Bool(true));
        assert_eq!(Value::from(123), Value::Int(123));
        assert_eq!(Value::from(1.23), Value::Float(1.23));
        assert_eq!(Value::from("str"), Value::String("str".to_string()));
        assert_eq!(
            Value::from("str".to_string()),
            Value::String("str".to_string())
        );
        assert_eq!(Value::from(None as Option<i64>), Value::Null,);
        assert_eq!(Value::from(Some("str")), Value::String("str".to_string()),);
        assert_eq!(
            Value::from(vec![
                Value::from("str"),
                Value::from(123),
                Value::from(vec![Value::from(true)]),
                Value::from(vec![1.23, 3.45, 5.22]),
            ]),
            Value::Array(vec![
                Value::String("str".to_string()),
                Value::Int(123),
                Value::Array(vec![Value::Bool(true)]),
                Value::Array(vec![
                    Value::Float(1.23),
                    Value::Float(3.45),
                    Value::Float(5.22)
                ]),
            ]),
        );
        assert_eq!(
            Value::from(BTreeMap::from([
                ("str", Value::from("str")),
                ("num", Value::from(3.14)),
                ("arr", Value::from(vec!["item_1"])),
            ])),
            Value::Object(BTreeMap::from([
                ("str".to_string(), Value::String("str".to_string())),
                ("num".to_string(), Value::Float(3.14)),
                (
                    "arr".to_string(),
                    Value::Array(vec![Value::String("item_1".to_string())])
                ),
            ])),
        );
        assert_eq!(
            Value::from(HashMap::from([
                ("str", Value::from("str")),
                ("num", Value::from(3.14)),
                ("arr", Value::from(vec!["item_1"])),
            ])),
            Value::Object(BTreeMap::from([
                ("str".to_string(), Value::String("str".to_string())),
                ("num".to_string(), Value::Float(3.14)),
                (
                    "arr".to_string(),
                    Value::Array(vec![Value::String("item_1".to_string())])
                ),
            ])),
        );
    }
}
