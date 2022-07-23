use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
    domain::{Error, Id, Page, Schema, SchemaRepository},
    infrastructure::SqliteSchema,
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
              name TEXT,
              root_prop JSON,
              configs JSON,
              created_at TIMESTAMP WITH TIME ZONE,
              updated_at TIMESTAMP WITH TIME ZONE,
              version INTEGER
            )
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

        let sqlite_schemas: Vec<SqliteSchema> = sqlx::query_as(
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

        let count: u32 = sqlx::query_scalar("SELECT COUNT(*) FROM schemas")
            .fetch_one(&self.pool)
            .await
            .map_err(Error::Database)?;

        Page::new(
            offset,
            limit,
            count as u64,
            sqlite_schemas
                .into_iter()
                .map(SqliteSchema::to_domain)
                .collect::<Result<Vec<Schema>, Error>>()?,
        )
    }

    async fn find_by_id(&self, id: &Id) -> Result<Option<Schema>, Error> {
        let sqlite_schema: Option<SqliteSchema> =
            sqlx::query_as("SELECT * FROM schemas WHERE id = $1")
                .bind(id.value())
                .fetch_optional(&self.pool)
                .await
                .map_err(Error::Database)?;

        Ok(sqlite_schema.map(SqliteSchema::to_domain).transpose()?)
    }

    async fn exists(&self, id: &Id) -> Result<bool, Error> {
        Ok(self.find_by_id(id).await?.is_some())
    }

    async fn save(&self, schema: &mut Schema) -> Result<(), Error> {
        let is_new = schema.timestamps().created_at() == schema.timestamps().updated_at();

        let sqlite_schema = SqliteSchema::from_domain(schema)?;

        let query = if is_new {
            sqlx::query(
                "
                INSERT INTO schemas(
                    id,
                    name,
                    root_prop,
                    configs,
                    created_at,
                    updated_at,
                    version
                ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                ",
            )
            .bind(sqlite_schema.id)
            .bind(sqlite_schema.name)
            .bind(sqlite_schema.root_prop)
            .bind(sqlite_schema.configs)
            .bind(sqlite_schema.created_at)
            .bind(sqlite_schema.updated_at)
            .bind(sqlite_schema.version)
        } else {
            sqlx::query(
                "
                UPDATE schemas
                SET
                    name = $2,
                    root_prop = $3,
                    configs = $4,
                    updated_at = $5,
                    version = $6
                WHERE id = $1
                ",
            )
            .bind(sqlite_schema.id)
            .bind(sqlite_schema.name)
            .bind(sqlite_schema.root_prop)
            .bind(sqlite_schema.configs)
            .bind(sqlite_schema.updated_at)
            .bind(sqlite_schema.version)
        };

        query.execute(&self.pool).await.map_err(Error::Database)?;

        Ok(())
    }

    async fn delete(&self, id: &Id) -> Result<(), Error> {
        sqlx::query("DELETE FROM schemas WHERE id = $1")
            .bind(id.value())
            .execute(&self.pool)
            .await
            .map_err(Error::Database)?;

        Ok(())
    }
}
