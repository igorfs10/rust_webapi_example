use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;

// Trait para forçar a criação do CRUD padrão, é necessário setar o IdType na implementação
// para definir o tipo que o id usa para essa entidade. Ex: u32 para INT, String para VARCHAR
#[async_trait]
pub trait BaseRepo {
    type IdType;
    type ModelType;

    async fn get_all() -> Result<Vec<Self::ModelType>, sqlx::Error>;
    async fn get_by_id(id: Self::IdType) -> Result<Self::ModelType, sqlx::Error>;
    async fn insert(data: Self::ModelType) -> Result<PgQueryResult, sqlx::Error>;
    async fn update(data: Self::ModelType) -> Result<PgQueryResult, sqlx::Error>;
    async fn delete(id: Self::IdType) -> Result<PgQueryResult, sqlx::Error>;
}
