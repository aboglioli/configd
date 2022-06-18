use regex::Regex;
use std::collections::BTreeMap;

use crate::domain::{Error, Interval, Kind, Value};

pub trait PropConverter<T> {
    type Error;

    fn from(&self, props: T) -> Result<Prop, Self::Error>;
    fn to(&self, prop: Prop) -> Result<T, Self::Error>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Prop {
    Bool {
        required: bool,
        default_value: Option<Value>,
    },
    Int {
        required: bool,
        default_value: Option<Value>,
        allowed_values: Option<Vec<Value>>,
        interval: Option<Interval>,
    },
    Float {
        required: bool,
        default_value: Option<Value>,
        allowed_values: Option<Vec<Value>>,
        interval: Option<Interval>,
    },
    String {
        required: bool,
        default_value: Option<Value>,
        allowed_values: Option<Vec<Value>>,
        regex: Option<String>,
    },
    Array(Box<Prop>),
    Object(BTreeMap<String, Prop>),
}

impl Prop {
    pub fn bool(required: bool, default_value: Option<Value>) -> Result<Prop, Error> {
        if let Some(default_value) = &default_value {
            if default_value.kind() != Kind::Bool {
                return Err(Error::Generic);
            }
        }

        Ok(Prop::Bool {
            required,
            default_value,
        })
    }

    pub fn int(
        required: bool,
        default_value: Option<Value>,
        allowed_values: Option<Vec<Value>>,
        interval: Option<Interval>,
    ) -> Result<Prop, Error> {
        if let Some(default_value) = &default_value {
            if default_value.kind() != Kind::Int {
                return Err(Error::Generic);
            }
        }

        if let Some(allowed_values) = &allowed_values {
            if allowed_values.iter().any(|value| value.kind() != Kind::Int) {
                return Err(Error::Generic);
            }
        }

        Ok(Prop::Int {
            required,
            default_value,
            allowed_values,
            interval,
        })
    }

    pub fn float(
        required: bool,
        default_value: Option<Value>,
        allowed_values: Option<Vec<Value>>,
        interval: Option<Interval>,
    ) -> Result<Prop, Error> {
        if let Some(default_value) = &default_value {
            if default_value.kind() != Kind::Float {
                return Err(Error::Generic);
            }
        }

        if let Some(allowed_values) = &allowed_values {
            if allowed_values
                .iter()
                .any(|value| value.kind() != Kind::Float)
            {
                return Err(Error::Generic);
            }
        }

        Ok(Prop::Float {
            required,
            default_value,
            allowed_values,
            interval,
        })
    }

    pub fn string(
        required: bool,
        default_value: Option<Value>,
        allowed_values: Option<Vec<Value>>,
        regex: Option<String>,
    ) -> Result<Prop, Error> {
        if let Some(default_value) = &default_value {
            if default_value.kind() != Kind::String {
                return Err(Error::Generic);
            }
        }

        if let Some(allowed_values) = &allowed_values {
            if allowed_values
                .iter()
                .any(|value| value.kind() != Kind::String)
            {
                return Err(Error::Generic);
            }
        }

        Ok(Prop::String {
            required,
            default_value,
            allowed_values,
            regex,
        })
    }

    pub fn array(prop: Prop) -> Prop {
        Prop::Array(Box::new(prop))
    }

    pub fn object(props: BTreeMap<String, Prop>) -> Prop {
        Prop::Object(props)
    }

    pub fn is_required(&self) -> bool {
        match self {
            Prop::Bool { required, .. }
            | Prop::Int { required, .. }
            | Prop::Float { required, .. }
            | Prop::String { required, .. } => *required,
            _ => true,
        }
    }
    pub fn default_value(&self) -> Option<&Value> {
        match self {
            Prop::Bool { default_value, .. }
            | Prop::Int { default_value, .. }
            | Prop::Float { default_value, .. }
            | Prop::String { default_value, .. } => default_value.as_ref(),
            _ => None,
        }
    }

    pub fn allowed_values(&self) -> Option<&[Value]> {
        match self {
            Prop::Int { allowed_values, .. }
            | Prop::Float { allowed_values, .. }
            | Prop::String { allowed_values, .. } => allowed_values.as_deref(),
            _ => None,
        }
    }

    pub fn interval(&self) -> Option<&Interval> {
        match self {
            Prop::Int { interval, .. } | Prop::Float { interval, .. } => interval.as_ref(),
            _ => None,
        }
    }

    pub fn regex(&self) -> Option<&str> {
        match self {
            Prop::String { regex, .. } => regex.as_deref(),
            _ => None,
        }
    }

    pub fn validate(&self, value: &Value) -> bool {
        if value == &Value::Null {
            return !self.is_required();
        }

        if let Some(allowed_values) = self.allowed_values() {
            println!("allowed values {:?}", allowed_values);
            if allowed_values.iter().all(|v| v != value) {
                println!("NOO");
                return false;
            }
        }

        match self {
            Prop::Bool { .. } => true,
            Prop::Int { interval, .. } => {
                if let Value::Int(value) = value {
                    if let Some(interval) = interval {
                        return interval.validate(*value as f64);
                    }

                    return true;
                }

                false
            }
            Prop::Float { interval, .. } => {
                if let Value::Float(value) = value {
                    if let Some(interval) = interval {
                        return interval.validate(*value);
                    }

                    return true;
                }

                false
            }
            Prop::String { regex, .. } => {
                if let Value::String(value) = value {
                    if let Some(regex) = regex {
                        if let Ok(regex) = Regex::new(regex) {
                            return regex.is_match(value);
                        }
                    }

                    return true;
                }

                false
            }
            Prop::Array(prop) => {
                if let Value::Array(items) = value {
                    return items.iter().all(|item| prop.validate(item));
                }

                false
            }
            Prop::Object(props) => {
                if let Value::Object(object) = value {
                    return object.iter().all(|(key, value)| {
                        if let Some(prop) = props.get(key) {
                            return prop.validate(value);
                        }

                        false
                    }) && props.iter().all(|(key, _)| object.contains_key(key));
                }

                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
        assert!(Prop::bool(true, None)
            .unwrap()
            .validate(&Value::Bool(false)));

        // Required
        assert!(Prop::int(false, None, None, None)
            .unwrap()
            .validate(&Value::Null));
        assert!(Prop::string(false, None, None, None)
            .unwrap()
            .validate(&Value::Null));
        assert!(Prop::array(Prop::int(true, None, None, None).unwrap())
            .validate(&Value::Array(vec![Value::Int(12)])));
        assert!(Prop::array(Prop::int(false, None, None, None).unwrap())
            .validate(&Value::Array(vec![Value::Null])));
        assert!(!Prop::int(true, None, None, None)
            .unwrap()
            .validate(&Value::Null));
        assert!(!Prop::string(true, None, None, None)
            .unwrap()
            .validate(&Value::Null));
        assert!(!Prop::array(Prop::int(true, None, None, None).unwrap()).validate(&Value::Null));
        assert!(!Prop::array(Prop::int(true, None, None, None).unwrap())
            .validate(&Value::Array(vec![Value::Null])));

        // Allowed values
        assert!(Prop::string(
            true,
            None,
            Some(vec![
                Value::String("dev".to_string()),
                Value::String("stg".to_string()),
                Value::String("prod".to_string()),
            ]),
            None
        )
        .unwrap()
        .validate(&Value::String("dev".to_string())),);
        assert!(!Prop::string(
            true,
            None,
            Some(vec![
                Value::String("dev".to_string()),
                Value::String("stg".to_string()),
                Value::String("prod".to_string()),
            ]),
            None
        )
        .unwrap()
        .validate(&Value::String("other".to_string())));
        assert!(!Prop::string(
            true,
            None,
            Some(vec![
                Value::String("dev".to_string()),
                Value::String("stg".to_string()),
                Value::String("prod".to_string()),
            ]),
            None
        )
        .unwrap()
        .validate(&Value::Int(3)));

        // Interval
        assert!(
            Prop::int(true, None, None, Some(Interval::new(1, 5).unwrap()))
                .unwrap()
                .validate(&Value::Int(3))
        );
        assert!(
            !Prop::int(true, None, None, Some(Interval::new(1, 5).unwrap()))
                .unwrap()
                .validate(&Value::Int(6))
        );

        // Regex
        assert!(Prop::string(
            true,
            None,
            None,
            Some("^http://[a-z]+:[0-9]{2,4}$".to_string())
        )
        .unwrap()
        .validate(&Value::String("http://localhost:8080".to_string())));
        assert!(!Prop::string(
            true,
            None,
            None,
            Some("^http://[a-z]+:[0-9]{2,4}$".to_string())
        )
        .unwrap()
        .validate(&Value::String("http://loc4lh0st:8080".to_string())));
        assert!(!Prop::string(
            true,
            None,
            None,
            Some("^http://[a-z]+:[0-9]{2,4}$".to_string())
        )
        .unwrap()
        .validate(&Value::String("localhost:8080".to_string())));
    }
}
