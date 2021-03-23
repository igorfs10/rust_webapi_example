use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;

#[async_trait]
pub trait CRUD<T: Sized> {
    type IdType;
    type TableType;

    async fn get_all() -> Result<Vec<Self::TableType>, sqlx::Error>;
    async fn get_by_id(id: Self::IdType) -> Result<Self::TableType, sqlx::Error>;
    async fn add(data: Self::TableType) -> Result<PgQueryResult, sqlx::Error>;
    async fn update(data: Self::TableType) -> Result<PgQueryResult, sqlx::Error>;
    async fn remove(id: Self::IdType) -> Result<PgQueryResult, sqlx::Error>;
}
