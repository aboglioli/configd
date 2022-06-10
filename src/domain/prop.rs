use regex::Regex;
use std::collections::HashMap;

use crate::domain::{Error, Interval, Kind, Value};

pub struct Prop {
    key: String,
    kind: Kind,

    required: bool,
    default_value: Option<Value>,
    allowed_values: Option<Vec<Value>>,
    regex: Option<Regex>,
    interval: Option<Interval>,

    array: bool,

    props: Option<HashMap<String, Prop>>,
}

impl Prop {
    pub fn new<K: Into<String>>(
        key: K,
        kind: Kind,
        required: bool,
        default_value: Option<Value>,
        allowed_values: Option<Vec<Value>>,
        regex: Option<String>,
        interval: Option<Interval>,
        array: bool,
        props: Option<Vec<Prop>>,
    ) -> Result<Prop, Error> {
        let key = key.into();

        if key.is_empty() {
            return Err(Error::Generic);
        }

        if kind == Kind::Null || kind == Kind::Array {
            return Err(Error::Generic);
        }

        let mut prop = Prop {
            key,
            kind,
            required: false,
            default_value: None,
            allowed_values: None,
            regex: None,
            interval: None,
            array: false,
            props: None,
        };

        if required {
            prop = prop.mark_as_required();
        }

        if let Some(value) = default_value {
            prop = prop.set_default_value(value)?;
        }

        if let Some(allowed_values) = allowed_values {
            prop = prop.add_allowed_values(allowed_values)?;
        }

        if let Some(regex) = regex {
            prop = prop.set_regex(regex)?;
        }

        if let Some(interval) = interval {
            prop = prop.set_interval(interval)?;
        }

        if array {
            prop = prop.mark_as_array();
        }

        if let Some(props) = props {
            prop = prop.add_props(props)?;
        }

        Ok(prop)
    }

    pub fn create<K: Into<String>>(key: K, kind: Kind) -> Result<Prop, Error> {
        Prop::new(key, kind, false, None, None, None, None, false, None)
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    pub fn default_value(&self) -> Option<&Value> {
        self.default_value.as_ref()
    }

    pub fn allowed_values(&self) -> Option<&Vec<Value>> {
        self.allowed_values.as_ref()
    }

    pub fn regex(&self) -> Option<&Regex> {
        self.regex.as_ref()
    }

    pub fn interval(&self) -> Option<&Interval> {
        self.interval.as_ref()
    }

    pub fn is_array(&self) -> bool {
        self.array
    }

    pub fn props(&self) -> Option<&HashMap<String, Prop>> {
        self.props.as_ref()
    }

    pub fn is_object(&self) -> bool {
        self.props.is_some()
    }

    // Builder
    pub fn mark_as_required(mut self) -> Prop {
        self.required = true;
        self
    }

    pub fn set_default_value(mut self, value: Value) -> Result<Prop, Error> {
        if self.kind != value.kind() {
            return Err(Error::Generic);
        }

        self.default_value = Some(value);

        Ok(self)
    }

    pub fn add_allowed_values(mut self, values: Vec<Value>) -> Result<Prop, Error> {
        if values.iter().any(|v| self.kind != v.kind()) {
            return Err(Error::Generic);
        }

        let mut allowed_values = self.allowed_values.unwrap_or_else(Vec::new);
        allowed_values.extend(values);
        self.allowed_values = Some(allowed_values);

        Ok(self)
    }

    pub fn set_regex<S: AsRef<str>>(mut self, pattern: S) -> Result<Prop, Error> {
        if self.kind != Kind::String {
            return Err(Error::Generic);
        }

        let re = Regex::new(pattern.as_ref()).map_err(|_| Error::Generic)?;

        self.regex = Some(re);

        Ok(self)
    }

    pub fn set_interval(mut self, interval: Interval) -> Result<Prop, Error> {
        if self.kind != Kind::Int && self.kind != Kind::Float {
            return Err(Error::Generic);
        }

        self.interval = Some(interval);

        Ok(self)
    }

    pub fn mark_as_array(mut self) -> Prop {
        self.array = true;
        self
    }

    pub fn add_props(mut self, props: Vec<Prop>) -> Result<Prop, Error> {
        if self.kind != Kind::Object {
            return Err(Error::Generic);
        }

        let mut existing_props = self.props.unwrap_or_else(HashMap::new);
        for prop in props.into_iter() {
            existing_props.insert(prop.key().to_string(), prop);
        }
        self.props = Some(existing_props);

        Ok(self)
    }

    // Value validation
    pub fn validate(&self, value: &Value) -> Result<(), Error> {
        self.validate_with_array(value, true)
    }

    fn validate_with_array(&self, value: &Value, validate_array: bool) -> Result<(), Error> {
        // Array
        if validate_array && self.is_array() {
            if let Value::Array(items) = value {
                for item in items.iter() {
                    self.validate_with_array(item, false)?;
                }

                return Ok(());
            } else {
                return Err(Error::Generic);
            }
        }

        // Object
        if self.is_object() {
            if let Value::Object(object) = value {
                if let Some(props) = &self.props {
                    for (key, value) in object.iter() {
                        if let Some(prop) = props.get(key) {
                            prop.validate_with_array(value, true)?;
                        } else {
                            return Err(Error::Generic);
                        }
                    }

                    for (key, _) in props.iter() {
                        if !object.contains_key(key) {
                            return Err(Error::Generic);
                        }
                    }

                    return Ok(());
                }
            }

            return Err(Error::Generic);
        }

        // Value
        if self.kind != value.kind() && value.kind() != Kind::Null {
            return Err(Error::Generic);
        }

        if self.required && value == &Value::Null {
            return Err(Error::Generic);
        }

        if let Some(allowed_values) = &self.allowed_values {
            if allowed_values.iter().all(|v| v != value) {
                return Err(Error::Generic);
            }
        }

        if let Some(regex) = &self.regex {
            match value {
                Value::String(value) => {
                    if !regex.is_match(value) {
                        return Err(Error::Generic);
                    }
                }
                _ => return Err(Error::Generic),
            }
        }

        if let Some(interval) = &self.interval {
            if !interval.is_value_valid(value) {
                return Err(Error::Generic);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let prop = Prop::create("prop", Kind::String).unwrap();
        assert_eq!(prop.key(), "prop");
        assert_eq!(prop.kind(), &Kind::String);

        // Valid
        assert!(Prop::create("prop", Kind::String)
            .unwrap()
            .set_regex("[a-z]*")
            .is_ok());
        assert!(Prop::create("prop", Kind::Object)
            .unwrap()
            .add_props(vec![Prop::create("subprop", Kind::Int).unwrap()])
            .is_ok());

        // Invalid
        assert!(Prop::create("prop", Kind::String)
            .unwrap()
            .set_default_value(Value::Int(3))
            .is_err());
        assert!(Prop::create("prop", Kind::String)
            .unwrap()
            .add_allowed_values(vec![Value::String("default".to_string()), Value::Int(3)])
            .is_err());
        assert!(Prop::create("prop", Kind::Int)
            .unwrap()
            .set_regex("[a-z]*")
            .is_err());
        assert!(Prop::create("prop", Kind::String)
            .unwrap()
            .set_interval(Interval::new(1, 3).unwrap())
            .is_err());
        assert!(Prop::create("prop", Kind::String)
            .unwrap()
            .add_props(vec![Prop::create("subprop", Kind::Int).unwrap()])
            .is_err());
        assert!(Prop::create("prop", Kind::Object)
            .unwrap()
            .set_regex("[a-zA-Z]+")
            .is_err());
    }

    #[test]
    fn validate() {
        // Valid
        assert!(Prop::create("prop", Kind::String)
            .unwrap()
            .set_regex("[a-z]*")
            .unwrap()
            .validate(&Value::String("hello".to_string()))
            .is_ok());
        assert!(Prop::create("prop", Kind::Int)
            .unwrap()
            .set_interval(Interval::new(1, 5).unwrap())
            .unwrap()
            .validate(&Value::Int(3))
            .is_ok());
        assert!(Prop::create("prop", Kind::Int)
            .unwrap()
            .add_allowed_values(vec![Value::Int(1), Value::Int(2)])
            .unwrap()
            .validate(&Value::Int(2))
            .is_ok());
        assert!(Prop::create("prop", Kind::Int)
            .unwrap()
            .mark_as_array()
            .set_interval(Interval::new(1, 5).unwrap())
            .unwrap()
            .validate(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
            .is_ok());
        assert!(Prop::create("prop", Kind::Float)
            .unwrap()
            .mark_as_array()
            .validate(&Value::Array(vec![Value::Float(3.14), Value::Null]))
            .is_ok());
        assert!(Prop::create("prop", Kind::Object)
            .unwrap()
            .add_props(vec![
                Prop::create("str", Kind::String).unwrap(),
                Prop::create("num", Kind::Int).unwrap(),
                Prop::create("bool", Kind::Bool).unwrap(),
            ])
            .unwrap()
            .validate(&Value::Object(HashMap::from([
                ("str".to_string(), Value::String("String".to_string())),
                ("num".to_string(), Value::Int(2)),
                ("bool".to_string(), Value::Bool(true)),
            ]),))
            .is_ok());

        // Invalid
        assert!(Prop::create("prop", Kind::String)
            .unwrap()
            .validate(&Value::Int(123))
            .is_err());
        assert!(Prop::create("prop", Kind::String)
            .unwrap()
            .mark_as_required()
            .validate(&Value::Null)
            .is_err());
        assert!(Prop::create("prop", Kind::String)
            .unwrap()
            .set_regex("^[a-z]*$")
            .unwrap()
            .validate(&Value::String("Hello123".to_string()))
            .is_err());
        assert!(Prop::create("prop", Kind::Int)
            .unwrap()
            .set_interval(Interval::new(1, 5).unwrap())
            .unwrap()
            .validate(&Value::Int(6))
            .is_err());
        assert!(Prop::create("prop", Kind::Int)
            .unwrap()
            .add_allowed_values(vec![Value::Int(1), Value::Int(3)])
            .unwrap()
            .validate(&Value::Int(2))
            .is_err());
        assert!(Prop::create("prop", Kind::Int)
            .unwrap()
            .mark_as_array()
            .validate(&Value::Array(vec![
                Value::Int(1),
                Value::String("2".to_string())
            ]))
            .is_err());
        assert!(Prop::create("prop", Kind::Int)
            .unwrap()
            .mark_as_array()
            .set_interval(Interval::new(1, 5).unwrap())
            .unwrap()
            .validate(&Value::Array(vec![Value::Int(1), Value::Int(6)]))
            .is_err());
        assert!(Prop::create("prop", Kind::Float)
            .unwrap()
            .mark_as_array()
            .mark_as_required()
            .validate(&Value::Array(vec![Value::Float(3.14), Value::Null]))
            .is_err());
        assert!(Prop::create("prop", Kind::Object)
            .unwrap()
            .add_props(vec![
                Prop::create("str", Kind::String).unwrap(),
                Prop::create("num", Kind::Int).unwrap(),
                Prop::create("bool", Kind::Bool).unwrap(),
            ])
            .unwrap()
            .validate(&Value::Object(HashMap::from([
                ("str".to_string(), Value::Int(2)),
                ("num".to_string(), Value::String("String".to_string())),
                ("bool".to_string(), Value::Bool(true)),
            ])))
            .is_err());
        assert!(Prop::create("prop", Kind::Object)
            .unwrap()
            .add_props(vec![
                Prop::create("str", Kind::String).unwrap(),
                Prop::create("num", Kind::Int).unwrap().mark_as_required(),
            ])
            .unwrap()
            .validate(&Value::Object(HashMap::from([
                ("str".to_string(), Value::Null),
                ("num".to_string(), Value::Null),
            ])))
            .is_err());
        assert!(Prop::create("prop", Kind::Object)
            .unwrap()
            .add_props(vec![
                Prop::create("str", Kind::String).unwrap(),
                Prop::create("num", Kind::Int).unwrap(),
            ])
            .unwrap()
            .validate(&Value::Object(HashMap::from([
                ("str".to_string(), Value::String("String".to_string())),
                ("num".to_string(), Value::Int(2)),
                ("extra_prop".to_string(), Value::Bool(true)),
            ])))
            .is_err());
        assert!(Prop::create("prop", Kind::Object)
            .unwrap()
            .add_props(vec![
                Prop::create("str", Kind::String).unwrap(),
                Prop::create("num", Kind::Int).unwrap(),
                Prop::create("bool", Kind::Bool).unwrap(),
            ])
            .unwrap()
            .validate(&Value::Object(HashMap::from([
                ("str".to_string(), Value::String("String".to_string())),
                ("num".to_string(), Value::Int(2)),
            ]),))
            .is_err());
    }
}
