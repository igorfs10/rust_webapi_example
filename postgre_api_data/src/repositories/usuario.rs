use std::sync::Arc;

use crate::structs;
use crate::traits;

use async_trait::async_trait;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};

use structs::table::Table;
use structs::usuario::Usuario;
use traits::crud::Crud;

#[async_trait]
impl Crud for Table<Usuario> {
    type IdType = i64;
    type TableType = Usuario;

    async fn get_all(pool: Arc<Pool<Postgres>>) -> Result<Vec<Self::TableType>, sqlx::Error> {
        let result = sqlx::query_as("SELECT * FROM USUARIOS;")
            .fetch_all(&*pool)
            .await;
        result
    }

    async fn get_by_id(
        pool: Arc<Pool<Postgres>>,
        id: Self::IdType,
    ) -> Result<Self::TableType, sqlx::Error> {
        let result = sqlx::query_as("SELECT * FROM USUARIOS WHERE ID = $1;")
            .bind(id)
            .fetch_one(&*pool)
            .await;
        result
    }

    async fn add(
        pool: Arc<Pool<Postgres>>,
        data: Self::TableType,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let result = sqlx::query("INSERT INTO USUARIOS (NOME) VALUES($1);")
            .bind(data.nome)
            .execute(&*pool)
            .await;
        result
    }

    async fn update(
        pool: Arc<Pool<Postgres>>,
        data: Self::TableType,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let result = sqlx::query("UPDATE USUARIOS SET NOME = $1 WHERE ID = $2;")
            .bind(data.nome)
            .bind(data.id)
            .execute(&*pool)
            .await;
        result
    }

    async fn remove(
        pool: Arc<Pool<Postgres>>,
        id: Self::IdType,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let result = sqlx::query("DELETE FROM USUARIOS WHERE ID = $1;")
            .bind(id)
            .execute(&*pool)
            .await;
        result
    }
}
