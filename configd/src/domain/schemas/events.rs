use core_lib::events::Publishable;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// Schema
#[derive(Serialize, Deserialize)]
pub struct SchemaCreated {
    pub id: String,
    pub name: String,
    pub root_prop: JsonValue,
}

impl Publishable for SchemaCreated {
    fn id(&self) -> &str {
        &self.id
    }

    fn topic(&self) -> &str {
        "schema.created"
    }
}

#[derive(Serialize, Deserialize)]
pub struct SchemaRootPropChanged {
    pub id: String,
    pub root_prop: JsonValue,
}

impl Publishable for SchemaRootPropChanged {
    fn id(&self) -> &str {
        &self.id
    }

    fn topic(&self) -> &str {
        "schema.root_prop_changed"
    }
}

#[derive(Serialize, Deserialize)]
pub struct SchemaDeleted {
    pub id: String,
}

impl Publishable for SchemaDeleted {
    fn id(&self) -> &str {
        &self.id
    }

    fn topic(&self) -> &str {
        "schema.deleted"
    }
}

// Config
#[derive(Serialize, Deserialize)]
pub struct ConfigAccessed {
    pub id: String,
    pub schema_id: String,
    pub source: String,
    pub instance: String,
}

impl Publishable for ConfigAccessed {
    fn id(&self) -> &str {
        &self.id
    }

    fn topic(&self) -> &str {
        "config.accessed"
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigCreated {
    pub id: String,
    pub schema_id: String,
    pub name: String,
    pub data: JsonValue,
    pub valid: bool,
}

impl Publishable for ConfigCreated {
    fn id(&self) -> &str {
        &self.id
    }

    fn topic(&self) -> &str {
        "config.created"
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigDataChanged {
    pub id: String,
    pub schema_id: String,
    pub data: JsonValue,
    pub valid: bool,
}

impl Publishable for ConfigDataChanged {
    fn id(&self) -> &str {
        &self.id
    }

    fn topic(&self) -> &str {
        "config.data_changed"
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigDeleted {
    pub id: String,
    pub schema_id: String,
}

impl Publishable for ConfigDeleted {
    fn id(&self) -> &str {
        &self.id
    }

    fn topic(&self) -> &str {
        "config.deleted"
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigAccessRemoved {
    pub id: String,
    pub schema_id: String,
    pub source: String,
    pub instance: String,
}

impl Publishable for ConfigAccessRemoved {
    fn id(&self) -> &str {
        &self.id
    }

    fn topic(&self) -> &str {
        "config.access_removed"
    }
}
