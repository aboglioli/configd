use std::collections::HashMap;

use crate::domain::Error;

#[derive(Debug, PartialEq)]
pub enum Kind {
    String,
    Int,
    Float,
    Bool,
    Object,
    Null,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Object(HashMap<String, Value>),
    Null,
}

impl Value {
    pub fn kind(&self) -> Kind {
        match self {
            Value::String(_) => Kind::String,
            Value::Int(_) => Kind::Int,
            Value::Float(_) => Kind::Float,
            Value::Bool(_) => Kind::Bool,
            Value::Object(_) => Kind::Object,
            Value::Null => Kind::Null,
        }
    }
}

pub struct Interval {
    min: Option<f64>,
    max: Option<f64>,
}

impl Interval {
    pub fn new<MIN, MAX>(min: MIN, max: MAX) -> Result<Interval, Error>
    where
        MIN: Into<Option<f64>>,
        MAX: Into<Option<f64>>,
    {
        let min = min.into();
        let max = max.into();

        if min.is_none() && max.is_none() {
            return Err(Error::Generic);
        }

        Ok(Interval { min, max })
    }

    pub fn min(&self) -> Option<f64> {
        self.min
    }

    pub fn max(&self) -> Option<f64> {
        self.max
    }

    pub fn is_value_valid(&self, value: &Value) -> bool {
        let value = match value {
            Value::Int(value) => *value as f64,
            Value::Float(value) => *value,
            _ => return false,
        };

        if let Some(min) = self.min {
            if value < min {
                return false;
            }
        }

        if let Some(max) = self.max {
            if value > max {
                return false;
            }
        }

        true
    }
}
