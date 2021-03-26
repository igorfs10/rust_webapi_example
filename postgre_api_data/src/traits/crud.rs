use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};

#[async_trait]
pub trait Crud {
    type IdType;
    type TableType;

    async fn get_all(pool: Arc<Pool<Postgres>>) -> Result<Vec<Self::TableType>, sqlx::Error>;
    async fn get_by_id(
        pool: Arc<Pool<Postgres>>,
        id: Self::IdType,
    ) -> Result<Self::TableType, sqlx::Error>;
    async fn add(
        pool: Arc<Pool<Postgres>>,
        data: Self::TableType,
    ) -> Result<PgQueryResult, sqlx::Error>;
    async fn update(
        pool: Arc<Pool<Postgres>>,
        data: Self::TableType,
    ) -> Result<PgQueryResult, sqlx::Error>;
    async fn remove(
        pool: Arc<Pool<Postgres>>,
        id: Self::IdType,
    ) -> Result<PgQueryResult, sqlx::Error>;
}
