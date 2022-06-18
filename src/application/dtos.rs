use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct SchemaDto {
    pub id: String,
    pub name: String,
    pub schema: Value,
}
