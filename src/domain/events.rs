use core_lib::events::Publishable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SchemaCreated {
    pub id: String,
    pub name: String,
    // pub root_prop: JsonValue,
}

impl Publishable for SchemaCreated {
    fn id(&self) -> &str {
        &self.id
    }

    fn topic(&self) -> &str {
        "schema.created"
    }
}
