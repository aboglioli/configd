use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::domain::{ConfigRepository, Error, Id};

#[derive(Deserialize)]
pub struct DeleteConfigCommand {
    pub schema_id: String,
    pub config_id: String,
}

#[derive(Serialize)]
pub struct DeleteConfigResponse {
    pub schema_id: String,
    pub config_id: String,
}

pub struct DeleteConfig {
    config_repository: Arc<dyn ConfigRepository + Sync + Send>,
}

impl DeleteConfig {
    pub fn new(config_repository: Arc<dyn ConfigRepository + Sync + Send>) -> DeleteConfig {
        DeleteConfig { config_repository }
    }

    pub async fn exec(&self, cmd: DeleteConfigCommand) -> Result<DeleteConfigResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;
        let config_id = Id::new(cmd.config_id)?;

        self.config_repository
            .delete(&schema_id, &config_id)
            .await?;

        Err(Error::Generic)
    }
}
