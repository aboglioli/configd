use crate::domain::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Id {
    id: String,
}

impl Id {
    pub fn new<I: Into<String>>(id: I) -> Result<Id, Error> {
        let id = id.into();

        if id.is_empty() {
            return Err(Error::Generic);
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

impl ToString for Id {
    fn to_string(&self) -> String {
        self.id.to_string()
    }
}
