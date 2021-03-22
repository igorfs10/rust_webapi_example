use sqlx::postgres::PgQueryResult;

pub trait CRUD<T: Sized> {
    type IdType;
    type TableType;

    fn get_all() -> Result<Vec<Self::TableType>, sqlx::Error>;
    fn get_by_id(id: Self::IdType) -> Result<Self::TableType, sqlx::Error>;
    fn add(data: Self::TableType) -> Result<PgQueryResult, sqlx::Error>;
    fn update(data: Self::TableType) -> Result<PgQueryResult, sqlx::Error>;
    fn remove(id: Self::IdType) -> Result<PgQueryResult, sqlx::Error>;
}
