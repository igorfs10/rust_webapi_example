use crate::structs;
use crate::traits;

use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;

use structs::usuario::Usuario;
use traits::crud::Crud;

use super::super::POOL;

#[async_trait]
impl Crud for Usuario {
    type IdType = i64;

    async fn get_all() -> Result<Vec<Self>, sqlx::Error> {
        let result = sqlx::query_as("SELECT * FROM USUARIOS;")
            .fetch_all(&**POOL)
            .await;
        result
    }

    async fn get_by_id(id: Self::IdType) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as("SELECT * FROM USUARIOS WHERE ID = $1;")
            .bind(id)
            .fetch_one(&**POOL)
            .await;
        result
    }

    async fn add(data: Self) -> Result<PgQueryResult, sqlx::Error> {
        let result = sqlx::query("INSERT INTO USUARIOS (NOME) VALUES($1);")
            .bind(data.nome)
            .execute(&**POOL)
            .await;
        result
    }

    async fn update(data: Self) -> Result<PgQueryResult, sqlx::Error> {
        let result = sqlx::query("UPDATE USUARIOS SET NOME = $1 WHERE ID = $2;")
            .bind(data.nome)
            .bind(data.id)
            .execute(&**POOL)
            .await;
        result
    }

    async fn remove(id: Self::IdType) -> Result<PgQueryResult, sqlx::Error> {
        let result = sqlx::query("DELETE FROM USUARIOS WHERE ID = $1;")
            .bind(id)
            .execute(&**POOL)
            .await;
        result
    }
}
