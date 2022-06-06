use regex::Regex;
use std::collections::HashMap;

use crate::domain::{Error, Interval, Kind, Value};

pub struct Prop {
    key: String,
    kind: Kind,

    array: bool,
    default_value: Option<Value>,
    required: bool,
    allowed_values: Option<Vec<Value>>,
    regex: Option<Regex>,
    interval: Option<Interval>,

    props: Option<HashMap<String, Prop>>,
}

impl Prop {
    pub fn new<K: Into<String>>(
        key: K,
        kind: Kind,
        array: bool,
        default_value: Option<Value>,
        required: bool,
        allowed_values: Option<Vec<Value>>,
        regex: Option<String>,
        interval: Option<Interval>,
        props: Option<HashMap<String, Prop>>,
    ) -> Result<Prop, Error> {
        let key = key.into();

        if key.is_empty() {
            return Err(Error::Generic);
        }

        let mut prop = Prop {
            key,
            array: false,
            default_value: None,
            kind,
            required: false,
            allowed_values: None,
            regex: None,
            interval: None,
            props: None,
        };

        if array {
            prop.mark_as_array();
        }

        if let Some(value) = default_value {
            prop.set_default_value(value)?;
        }

        if required {
            prop.mark_as_required();
        }

        if let Some(allowed_values) = allowed_values {
            prop.set_allowed_values(allowed_values)?;
        }

        if let Some(regex) = regex {
            prop.set_regex(regex)?;
        }

        if let Some(interval) = interval {
            prop.set_interval(interval)?;
        }

        if let Some(props) = props {
            prop.set_props(props)?;
        }

        Ok(prop)
    }

    pub fn create<K: Into<String>>(key: K, kind: Kind) -> Result<Prop, Error> {
        Prop::new(key, kind, false, None, false, None, None, None, None)
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

    pub fn mark_as_required(&mut self) {
        self.required = true;
    }

    pub fn is_array(&self) -> bool {
        self.array
    }

    pub fn mark_as_array(&mut self) {
        self.array = true;
    }

    pub fn default_value(&self) -> Option<&Value> {
        self.default_value.as_ref()
    }

    pub fn set_default_value(&mut self, value: Value) -> Result<(), Error> {
        if self.kind != value.kind() {
            return Err(Error::Generic);
        }

        self.default_value = Some(value);

        Ok(())
    }

    pub fn allowed_values(&self) -> Option<&Vec<Value>> {
        self.allowed_values.as_ref()
    }

    pub fn set_allowed_values(&mut self, values: Vec<Value>) -> Result<(), Error> {
        for value in values.iter() {
            if self.kind != value.kind() {
                return Err(Error::Generic);
            }
        }

        self.allowed_values = Some(values);

        Ok(())
    }

    pub fn add_allowed_value(&mut self, value: Value) -> Result<(), Error> {
        if self.kind != value.kind() {
            return Err(Error::Generic);
        }

        if let Some(allowed_values) = &mut self.allowed_values {
            allowed_values.push(value);
        } else {
            self.allowed_values = Some(vec![value]);
        }

        Ok(())
    }

    pub fn regex(&self) -> Option<&Regex> {
        self.regex.as_ref()
    }

    pub fn set_regex<S: AsRef<str>>(&mut self, pattern: S) -> Result<(), Error> {
        if self.kind != Kind::String {
            return Err(Error::Generic);
        }

        let re = Regex::new(pattern.as_ref()).map_err(|_| Error::Generic)?;

        self.regex = Some(re);

        Ok(())
    }

    pub fn interval(&self) -> Option<&Interval> {
        self.interval.as_ref()
    }

    pub fn set_interval(&mut self, interval: Interval) -> Result<(), Error> {
        if self.kind != Kind::Int && self.kind != Kind::Float {
            return Err(Error::Generic);
        }

        self.interval = Some(interval);

        Ok(())
    }

    pub fn props(&self) -> Option<&HashMap<String, Prop>> {
        self.props.as_ref()
    }

    pub fn set_props(&mut self, props: HashMap<String, Prop>) -> Result<(), Error> {
        if self.kind != Kind::Object {
            return Err(Error::Generic);
        }

        self.props = Some(props);

        Ok(())
    }

    pub fn add_prop<K: Into<String>>(&mut self, key: K, prop: Prop) -> Result<(), Error> {
        if self.kind != Kind::Object {
            return Err(Error::Generic);
        }

        if let Some(props) = &mut self.props {
            props.insert(key.into(), prop);
        } else {
            let mut props = HashMap::new();
            props.insert(key.into(), prop);

            self.props = Some(props);
        }

        Ok(())
    }

    pub fn validate_value(&self, value: &Value) -> Result<(), Error> {
        if self.kind != value.kind() {
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
    }
}
