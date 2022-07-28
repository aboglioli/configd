use async_trait::async_trait;
use sqlx::SqlitePool;
use std::collections::HashMap;

use crate::{
    domain::{
        configs::Access,
        errors::Error,
        schemas::{
            ConfigAccessRemoved, ConfigAccessed, ConfigCreated, ConfigDataChanged, ConfigDeleted,
            ConfigPasswordChanged, ConfigPasswordDeleted, ConfigRevalidated, Schema, SchemaCreated,
            SchemaDeleted, SchemaRepository, SchemaRootPropChanged,
        },
        shared::{Id, Page},
    },
    infrastructure::{SqlxAccess, SqlxConfig, SqlxSchema},
};

pub struct SQLiteSchemaRepository {
    pool: SqlitePool,
}

impl SQLiteSchemaRepository {
    pub async fn new(pool: SqlitePool) -> Result<SQLiteSchemaRepository, Error> {
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS schemas(
              id VARCHAR(255) PRIMARY KEY,
              name TEXT NOT NULL,
              root_prop JSON NOT NULL,
              created_at TIMESTAMP WITH TIME ZONE NOT NULL,
              updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
              version INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS configs(
              schema_id VARCHAR(255) NOT NULL,
              id VARCHAR(255) NOT NULL,
              name TEXT NOT NULL,
              data JSON NOT NULL,
              valid BOOLEAN NOT NULL,
              password TEXT,
              created_at TIMESTAMP WITH TIME ZONE NOT NULL,
              updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
              version INTEGER NOT NULL,
              PRIMARY KEY (schema_id, id)
            );

            CREATE TABLE IF NOT EXISTS accesses(
              schema_id VARCHAR(255) NOT NULL,
              id VARCHAR(255) NOT NULL,
              source TEXT NOT NULL,
              instance TEXT NOT NULL,
              timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
              previous TIMESTAMP WITH TIME ZONE,
              PRIMARY KEY (schema_id, id, source, instance)
            );
            ",
        )
        .execute(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(SQLiteSchemaRepository { pool })
    }
}

#[async_trait]
impl SchemaRepository for SQLiteSchemaRepository {
    async fn find(&self, offset: Option<u64>, limit: Option<u64>) -> Result<Page<Schema>, Error> {
        let offset = offset.unwrap_or(0);
        let mut limit = limit.unwrap_or(10);
        if limit > 25 {
            limit = 25;
        }

        let sqlite_schemas: Vec<SqlxSchema> = sqlx::query_as(
            "
                SELECT *
                FROM schemas
                LIMIT $1 OFFSET $2
           ",
        )
        .bind(limit as u32)
        .bind(offset as u32)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::Database)?;

        let mut schemas = Vec::new();
        for sqlite_schema in sqlite_schemas.into_iter() {
            let sqlite_configs: Vec<SqlxConfig> =
                sqlx::query_as("SELECT * FROM configs WHERE schema_id = $1")
                    .bind(&sqlite_schema.id)
                    .fetch_all(&self.pool)
                    .await
                    .map_err(Error::Database)?;

            let mut configs = HashMap::new();
            for sqlite_config in sqlite_configs.into_iter() {
                let sqlite_accesses: Vec<SqlxAccess> =
                    sqlx::query_as("SELECT * FROM accesses WHERE schema_id = $1 AND id = $2")
                        .bind(&sqlite_schema.id)
                        .bind(&sqlite_config.id)
                        .fetch_all(&self.pool)
                        .await
                        .map_err(Error::Database)?;

                let accesses = sqlite_accesses
                    .into_iter()
                    .map(SqlxAccess::to_domain)
                    .collect::<Result<Vec<Access>, Error>>()?;

                let config = sqlite_config.to_domain(accesses)?;
                configs.insert(config.id().clone(), config);
            }

            schemas.push(sqlite_schema.to_domain(configs)?);
        }

        let count: u32 = sqlx::query_scalar("SELECT COUNT(*) FROM schemas")
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        Page::new(offset, limit, count as u64, schemas)
    }

    async fn find_by_id(&self, id: &Id) -> Result<Option<Schema>, Error> {
        let sqlite_schema: Option<SqlxSchema> = sqlx::query_as("SELECT * FROM schemas")
            .bind(id.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(Error::Database)?;

        if let Some(sqlite_schema) = sqlite_schema {
            let sqlite_configs: Vec<SqlxConfig> =
                sqlx::query_as("SELECT * FROM configs WHERE schema_id = $1")
                    .bind(&sqlite_schema.id)
                    .fetch_all(&self.pool)
                    .await
                    .map_err(Error::Database)?;

            let mut configs = HashMap::new();
            for sqlite_config in sqlite_configs.into_iter() {
                let sqlite_accesses: Vec<SqlxAccess> =
                    sqlx::query_as("SELECT * FROM accesses WHERE schema_id = $1 AND id = $2")
                        .bind(&sqlite_schema.id)
                        .bind(&sqlite_config.id)
                        .fetch_all(&self.pool)
                        .await
                        .map_err(Error::Database)?;

                let accesses = sqlite_accesses
                    .into_iter()
                    .map(SqlxAccess::to_domain)
                    .collect::<Result<Vec<Access>, Error>>()?;

                let config = sqlite_config.to_domain(accesses)?;
                configs.insert(config.id().clone(), config);
            }

            Ok(Some(sqlite_schema.to_domain(configs)?))
        } else {
            Ok(None)
        }
    }

    async fn exists(&self, id: &Id) -> Result<bool, Error> {
        Ok(self.find_by_id(id).await?.is_some())
    }

    async fn save(&self, schema: &mut Schema) -> Result<(), Error> {
        for event in schema.all_events() {
            let query = match event.topic() {
                // Schemas
                "schema.created" => {
                    let payload: SchemaCreated = event.deserialize_payload().unwrap();

                    sqlx::query(
                        "
                        INSERT INTO schemas(
                            id,
                            name,
                            root_prop,
                            created_at,
                            updated_at,
                            version
                        ) VALUES ($1, $2, $3, $4, $5, 1)
                        ",
                    )
                    .bind(payload.id)
                    .bind(payload.name)
                    .bind(payload.root_prop)
                    .bind(event.timestamp())
                    .bind(event.timestamp())
                }
                "schema.root_prop_changed" => {
                    let payload: SchemaRootPropChanged = event.deserialize_payload().unwrap();

                    sqlx::query(
                        "
                        UPDATE schemas
                        SET
                            root_prop = $2,
                            updated_at = $3,
                            version = version + 1
                        WHERE id = $1
                        ",
                    )
                    .bind(payload.id)
                    .bind(payload.root_prop)
                    .bind(event.timestamp())
                }
                "schema.deleted" => {
                    let payload: SchemaDeleted = event.deserialize_payload().unwrap();

                    sqlx::query(
                        "
                        DELETE FROM schemas
                        WHERE id = $1
                        ",
                    )
                    .bind(payload.id)
                }
                // Configs
                "config.created" => {
                    let payload: ConfigCreated = event.deserialize_payload().unwrap();

                    sqlx::query(
                        "
                        INSERT INTO configs (
                            schema_id,
                            id,
                            name,
                            data,
                            valid,
                            password,
                            created_at,
                            updated_at,
                            version
                        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 1)
                        ",
                    )
                    .bind(payload.schema_id)
                    .bind(payload.id)
                    .bind(payload.name)
                    .bind(payload.data)
                    .bind(payload.valid)
                    .bind(payload.password)
                    .bind(event.timestamp())
                    .bind(event.timestamp())
                }
                "config.data_changed" => {
                    let payload: ConfigDataChanged = event.deserialize_payload().unwrap();

                    sqlx::query(
                        "
                        UPDATE configs
                        SET
                            data = $3,
                            valid = $4,
                            updated_at = $5,
                            version = version + 1
                        WHERE
                            schema_id = $1 AND id = $2
                        ",
                    )
                    .bind(payload.schema_id)
                    .bind(payload.id)
                    .bind(payload.data)
                    .bind(payload.valid)
                    .bind(event.timestamp())
                }
                "config.revalidated" => {
                    let payload: ConfigRevalidated = event.deserialize_payload().unwrap();

                    sqlx::query(
                        "
                        UPDATE configs
                        SET
                            valid = $3,
                            updated_at = $4,
                            version = version + 1
                        WHERE
                            schema_id = $1 AND id = $2
                        ",
                    )
                    .bind(payload.schema_id)
                    .bind(payload.id)
                    .bind(payload.valid)
                    .bind(event.timestamp())
                }
                "config.password_changed" => {
                    let payload: ConfigPasswordChanged = event.deserialize_payload().unwrap();

                    sqlx::query(
                        "
                        UPDATE configs
                        SET
                            password = $3,
                            updated_at = $4,
                            version = version + 1
                        WHERE
                            schema_id = $1 AND id = $2
                        ",
                    )
                    .bind(payload.schema_id)
                    .bind(payload.id)
                    .bind(payload.password)
                    .bind(event.timestamp())
                }
                "config.password_deleted" => {
                    let payload: ConfigPasswordDeleted = event.deserialize_payload().unwrap();

                    sqlx::query(
                        "
                        UPDATE configs
                        SET
                            password = null,
                            updated_at = $3,
                            version = version + 1
                        WHERE
                            schema_id = $1 AND id = $2
                        ",
                    )
                    .bind(payload.schema_id)
                    .bind(payload.id)
                    .bind(event.timestamp())
                }
                "config.deleted" => {
                    let payload: ConfigDeleted = event.deserialize_payload().unwrap();

                    sqlx::query(
                        "
                        DELETE FROM configs
                        WHERE schema_id = $1 AND id = $2
                        ",
                    )
                    .bind(payload.schema_id)
                    .bind(payload.id)
                }
                // Accesses
                "config.accessed" => {
                    let payload: ConfigAccessed = event.deserialize_payload().unwrap();

                    if let Some(previous) = payload.previous {
                        sqlx::query(
                            "
                            UPDATE accesses
                            SET
                                timestamp = $5,
                                previous = $6
                            WHERE
                                schema_id = $1
                                AND id = $2
                                AND source = $3
                                AND instance = $4
                            ",
                        )
                        .bind(payload.schema_id)
                        .bind(payload.id)
                        .bind(payload.source)
                        .bind(payload.instance)
                        .bind(payload.timestamp)
                        .bind(previous)
                    } else {
                        sqlx::query(
                            "
                            INSERT INTO accesses(
                                schema_id,
                                id,
                                source,
                                instance,
                                timestamp
                            ) VALUES ($1, $2, $3, $4, $5)
                            ",
                        )
                        .bind(payload.schema_id)
                        .bind(payload.id)
                        .bind(payload.source)
                        .bind(payload.instance)
                        .bind(payload.timestamp)
                    }
                }
                "config.access_removed" => {
                    let payload: ConfigAccessRemoved = event.deserialize_payload().unwrap();

                    sqlx::query(
                        "
                        DELETE FROM accesses
                        WHERE schema_id = $1 AND id = $2 AND source = $3 AND instance = $4
                        ",
                    )
                    .bind(payload.schema_id)
                    .bind(payload.id)
                    .bind(payload.source)
                    .bind(payload.instance)
                }
                _ => sqlx::query(
                    "
                        UPDATE schemas
                        SET
                            updated_at = $2
                        WHERE id = $1
                        ",
                )
                .bind(schema.id().value())
                .bind(schema.timestamps().updated_at()),
            };

            query.execute(&self.pool).await.map_err(Error::Database)?;
        }

        Ok(())
    }
}
