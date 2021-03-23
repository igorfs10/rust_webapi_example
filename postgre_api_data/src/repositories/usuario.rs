use crate::structs;
use crate::traits;
use crate::utils;

use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;

use structs::table::Table;
use structs::usuario::Usuario;
use traits::crud::CRUD;

#[async_trait]
impl CRUD<Usuario> for Table<Usuario> {
    type IdType = i64;
    type TableType = Usuario;

    async fn get_all() -> Result<Vec<Self::TableType>, sqlx::Error> {
        let pool = utils::connection::get_connection().await;
        let result = sqlx::query_as("SELECT * FROM USUARIOS;")
            .fetch_all(&pool)
            .await;
        pool.close().await;
        result
    }

    async fn get_by_id(id: Self::IdType) -> Result<Self::TableType, sqlx::Error> {
        let pool = utils::connection::get_connection().await;
        let result = sqlx::query_as("SELECT * FROM USUARIOS WHERE ID = $1;")
            .bind(id)
            .fetch_one(&pool)
            .await;
        pool.close().await;
        result
    }

    async fn add(data: Self::TableType) -> Result<PgQueryResult, sqlx::Error> {
        let pool = utils::connection::get_connection().await;
        let result = sqlx::query("INSERT INTO USUARIOS (NOME) VALUES($1);")
            .bind(data.nome)
            .execute(&pool)
            .await;
        pool.close().await;
        result
    }

    async fn update(data: Self::TableType) -> Result<PgQueryResult, sqlx::Error> {
        let pool = utils::connection::get_connection().await;
        let result = sqlx::query("UPDATE USUARIOS SET NOME = $1 WHERE ID = $2;")
            .bind(data.nome)
            .bind(data.id)
            .execute(&pool)
            .await;
        pool.close().await;
        result
    }

    async fn remove(id: Self::IdType) -> Result<PgQueryResult, sqlx::Error> {
        let pool = utils::connection::get_connection().await;
        let result = sqlx::query("DELETE FROM USUARIOS WHERE ID = $1;")
            .bind(id)
            .execute(&pool)
            .await;
        pool.close().await;
        result
    }
}
