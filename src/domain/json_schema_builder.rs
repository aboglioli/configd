use serde_json::Value as JsonValue;

use crate::domain::{Error, Schema, SchemaBuilder, SchemaId};

pub struct JsonSchemaBuilder;

impl JsonSchemaBuilder {
    pub fn new() -> JsonSchemaBuilder {
        JsonSchemaBuilder
    }
}

impl SchemaBuilder<String> for JsonSchemaBuilder {
    type Error = Error;

    fn build(&self, id: SchemaId, name: String, props: String) -> Result<Schema, Error> {
        let value: JsonValue = serde_json::from_str(&props).map_err(|_| Error::Generic)?;

        self.build(id, name, value)
    }
}

impl SchemaBuilder<JsonValue> for JsonSchemaBuilder {
    type Error = Error;

    fn build(&self, id: SchemaId, name: String, props: JsonValue) -> Result<Schema, Error> {
        Err(Error::Generic)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn build() {
//         let json_schema_builder = JsonSchemaBuilder::new();
//
//         let schema = json_schema_builder
//             .build(
//                 SchemaId::new("schema#01").unwrap(),
//                 "Schema 01".to_string(),
//                 r#"{
//                     "env": {
//                         "$schema": {
//                             "type": "string",
//                             "allowed_values": ["dev", "stg", "prod"]
//                         }
//                     }
//                 }"#
//                 .to_string(),
//             )
//             .unwrap();
//
//         assert_eq!(schema.id().value(), "schema#01");
//         assert_eq!(schema.name(), "Schema 01");
//         assert_eq!(schema.props().len(), 1);
//     }
// }
