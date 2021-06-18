use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;

// Trait para forçar a criação do CRUD padrão, é necessário setar o IdType na implementação
// para definir o tipo que o id usa para essa entidade. Ex: u32 para INT, String para VARCHAR
#[async_trait]
pub trait Crud: Sized {
    type IdType;

    async fn get_all() -> Result<Vec<Self>, sqlx::Error>;
    async fn get_by_id(id: Self::IdType) -> Result<Self, sqlx::Error>;
    async fn add(data: Self) -> Result<PgQueryResult, sqlx::Error>;
    async fn update(data: Self) -> Result<PgQueryResult, sqlx::Error>;
    async fn remove(id: Self::IdType) -> Result<PgQueryResult, sqlx::Error>;
}
