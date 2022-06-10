use std::collections::HashMap;

use crate::domain::Error;

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

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
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

pub struct Interval {
    min: Option<f64>,
    max: Option<f64>,
}

impl Interval {
    pub fn new<N, MIN, MAX>(min: MIN, max: MAX) -> Result<Interval, Error>
    where
        MIN: Into<Option<N>>,
        MAX: Into<Option<N>>,
        N: Into<f64>,
    {
        let min = min.into();
        let max = max.into();

        if min.is_none() && max.is_none() {
            return Err(Error::Generic);
        }

        Ok(Interval {
            min: min.map(|n| n.into()),
            max: max.map(|n| n.into()),
        })
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
