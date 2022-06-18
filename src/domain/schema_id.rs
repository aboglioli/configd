use crate::domain::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct SchemaId {
    id: String,
}

impl SchemaId {
    pub fn new<I: Into<String>>(id: I) -> Result<SchemaId, Error> {
        let id = id.into();

        if id.is_empty() {
            return Err(Error::Generic);
        }

        Ok(SchemaId { id })
    }

    pub fn slug<I: AsRef<str>>(id: I) -> Result<SchemaId, Error> {
        let id = slug::slugify(id);
        SchemaId::new(id)
    }

    pub fn value(&self) -> &str {
        &self.id
    }
}

impl ToString for SchemaId {
    fn to_string(&self) -> String {
        self.id.to_string()
    }
}
