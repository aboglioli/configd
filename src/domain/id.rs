use std::fmt;

use crate::domain::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id {
    id: String,
}

impl Id {
    pub fn new<I: Into<String>>(id: I) -> Result<Id, Error> {
        let id = id.into();

        if id.is_empty() {
            return Err(Error::EmptyId);
        }

        Ok(Id { id })
    }

    pub fn slug<I: AsRef<str>>(id: I) -> Result<Id, Error> {
        let id = slug::slugify(id);
        Id::new(id)
    }

    pub fn value(&self) -> &str {
        &self.id
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
