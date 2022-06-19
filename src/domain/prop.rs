use regex::Regex;
use std::collections::BTreeMap;

use crate::domain::{Diff, Error, Interval, Kind, Reason, Value};

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

    pub fn validate(&self, value: &Value) -> Diff {
        self.validate_with_key(value, "$".to_string())
    }

    fn validate_with_key(&self, value: &Value, key: String) -> Diff {
        let mut diff = Diff::new(key);

        // Null values
        if value == &Value::Null {
            if self.is_required() {
                diff.add(Reason::NullValue, None);
            }
        } else {
            // Allowed values
            if let Some(allowed_values) = self.allowed_values() {
                if allowed_values.iter().all(|v| v != value) {
                    diff.add(Reason::NotAllowedValue, None)
                }
            }

            // Check for each prop type
            match self {
                Prop::Bool { .. } => {
                    if value.kind() != Kind::Bool {
                        diff.add(Reason::NotABool, None);
                    }
                }
                Prop::Int { interval, .. } => {
                    if let Value::Int(value) = value {
                        if let Some(interval) = interval {
                            if !interval.validate(*value as f64) {
                                diff.add(Reason::NotInInterval, None)
                            }
                        }
                    } else {
                        diff.add(Reason::NotAnInt, None);
                    }
                }
                Prop::Float { interval, .. } => {
                    if let Value::Float(value) = value {
                        if let Some(interval) = interval {
                            if !interval.validate(*value) {
                                diff.add(Reason::NotInInterval, None)
                            }
                        }
                    } else {
                        diff.add(Reason::NotAFloat, None);
                    }
                }
                Prop::String { regex, .. } => {
                    if let Value::String(value) = value {
                        if let Some(regex) = regex {
                            if let Ok(regex) = Regex::new(regex) {
                                if !regex.is_match(value) {
                                    diff.add(Reason::UnmatchedRegex, None);
                                }
                            }
                        }
                    } else {
                        diff.add(Reason::NotAString, None);
                    }
                }
                Prop::Array(prop) => {
                    if let Value::Array(items) = value {
                        for (i, item) in items.iter().enumerate() {
                            diff.merge(prop.validate_with_key(item, i.to_string()));
                        }
                    } else {
                        diff.add(Reason::NotAnArray, None);
                    }
                }
                Prop::Object(props) => {
                    if let Value::Object(object) = value {
                        for (key, item) in object.iter() {
                            if let Some(prop) = props.get(key) {
                                diff.merge(prop.validate_with_key(item, key.to_string()))
                            } else {
                                diff.add(Reason::UnknownProp, Some(key.to_string()));
                            }
                        }

                        for key in props.keys() {
                            if !object.contains_key(key) {
                                diff.add(Reason::MissingProp, Some(key.to_string()));
                            }
                        }
                    } else {
                        diff.add(Reason::NotAnObject, None);
                    }
                }
            }
        }

        return diff;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn validate() {
        assert!(Prop::bool(true, None)
            .unwrap()
            .validate(&Value::Bool(false))
            .is_empty());

        // Required
        assert!(Prop::int(false, None, None, None)
            .unwrap()
            .validate(&Value::Null)
            .is_empty());
        assert!(Prop::string(false, None, None, None)
            .unwrap()
            .validate(&Value::Null)
            .is_empty());
        assert!(Prop::array(Prop::int(true, None, None, None).unwrap())
            .validate(&Value::Array(vec![Value::Int(12)]))
            .is_empty());
        assert!(Prop::array(Prop::int(false, None, None, None).unwrap())
            .validate(&Value::Array(vec![Value::Null]))
            .is_empty());
        assert!(!Prop::int(true, None, None, None)
            .unwrap()
            .validate(&Value::Null)
            .is_empty());
        assert!(!Prop::string(true, None, None, None)
            .unwrap()
            .validate(&Value::Null)
            .is_empty());
        assert!(!Prop::array(Prop::int(true, None, None, None).unwrap())
            .validate(&Value::Null)
            .is_empty());
        assert!(!Prop::array(Prop::int(true, None, None, None).unwrap())
            .validate(&Value::Array(vec![Value::Null]))
            .is_empty());

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
        .validate(&Value::String("dev".to_string()))
        .is_empty());
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
        .validate(&Value::String("other".to_string()))
        .is_empty());
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
        .validate(&Value::Int(3))
        .is_empty());

        // Interval
        assert!(
            Prop::int(true, None, None, Some(Interval::new(1, 5).unwrap()))
                .unwrap()
                .validate(&Value::Int(3))
                .is_empty()
        );
        assert!(
            !Prop::int(true, None, None, Some(Interval::new(1, 5).unwrap()))
                .unwrap()
                .validate(&Value::Int(6))
                .is_empty()
        );

        // Regex
        assert!(Prop::string(
            true,
            None,
            None,
            Some("^http://[a-z]+:[0-9]{2,4}$".to_string())
        )
        .unwrap()
        .validate(&Value::String("http://localhost:8080".to_string()))
        .is_empty());
        assert!(!Prop::string(
            true,
            None,
            None,
            Some("^http://[a-z]+:[0-9]{2,4}$".to_string())
        )
        .unwrap()
        .validate(&Value::String("http://loc4lh0st:8080".to_string()))
        .is_empty());
        assert!(!Prop::string(
            true,
            None,
            None,
            Some("^http://[a-z]+:[0-9]{2,4}$".to_string())
        )
        .unwrap()
        .validate(&Value::String("localhost:8080".to_string()))
        .is_empty());
    }

    #[test]
    fn generated_diff() {
        let prop = Prop::object(BTreeMap::from([
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
                "obj".to_string(),
                Prop::object(BTreeMap::from([
                    ("prop1".to_string(), Prop::bool(true, None).unwrap()),
                    (
                        "prop2".to_string(),
                        Prop::int(true, None, None, Some(Interval::new(1, 5).unwrap())).unwrap(),
                    ),
                    (
                        "prop3".to_string(),
                        Prop::float(
                            false,
                            None,
                            Some(vec![Value::Float(1.0), Value::Float(3.0)]),
                            None,
                        )
                        .unwrap(),
                    ),
                    (
                        "prop4".to_string(),
                        Prop::float(
                            false,
                            None,
                            Some(vec![Value::Float(1.0), Value::Float(3.0)]),
                            Some(Interval::new(1, 5).unwrap()),
                        )
                        .unwrap(),
                    ),
                ])),
            ),
            (
                "arr".to_string(),
                Prop::array(
                    Prop::string(true, None, None, Some("^asd[0-9]+$".to_string())).unwrap(),
                ),
            ),
        ]));

        let diff = prop.validate(&Value::Object(BTreeMap::from([
            ("env".to_string(), Value::String("local".to_string())),
            (
                "obj".to_string(),
                Value::Object(BTreeMap::from([
                    ("prop1".to_string(), Value::Null),
                    ("prop2".to_string(), Value::Int(6)),
                    ("prop4".to_string(), Value::Float(6.0)),
                    ("prop5".to_string(), Value::Int(1)),
                ])),
            ),
            (
                "arr".to_string(),
                Value::Array(vec![Value::String("asd".to_string()), Value::Null]),
            ),
        ])));

        assert_eq!(
            diff.diffs(),
            &HashMap::from([
                ("$.env".to_string(), vec![Reason::NotAllowedValue]),
                ("$.obj.prop1".to_string(), vec![Reason::NullValue]),
                ("$.obj.prop2".to_string(), vec![Reason::NotInInterval]),
                ("$.obj.prop3".to_string(), vec![Reason::MissingProp]),
                (
                    "$.obj.prop4".to_string(),
                    vec![Reason::NotAllowedValue, Reason::NotInInterval]
                ),
                ("$.obj.prop5".to_string(), vec![Reason::UnknownProp]),
                ("$.arr.0".to_string(), vec![Reason::UnmatchedRegex]),
                ("$.arr.1".to_string(), vec![Reason::NullValue]),
            ])
        )
    }
}
