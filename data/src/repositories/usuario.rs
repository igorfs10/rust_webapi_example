use crate::structs;
use crate::traits;

use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

use structs::usuario::Usuario;
use traits::base_repo::BaseRepo;

use crate::DB_CONNECTION_POOL;

// Mapeamento das colunas do banco para a struct
impl<'c> FromRow<'c, PgRow> for Usuario {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Usuario {
            id: row.get(0),
            nome: row.get(1),
        })
    }
}

#[async_trait]
impl BaseRepo for Usuario {
    type IdType = i64;

    async fn get_all() -> Result<Vec<Self>, sqlx::Error> {
        let result = sqlx::query_as("SELECT * FROM USUARIOS;")
            .fetch_all(&**DB_CONNECTION_POOL)
            .await;
        result
    }

    async fn get_by_id(id: Self::IdType) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as("SELECT * FROM USUARIOS WHERE ID = $1;")
            .bind(id)
            .fetch_one(&**DB_CONNECTION_POOL)
            .await;
        result
    }

    async fn insert(data: Self) -> Result<PgQueryResult, sqlx::Error> {
        let result = sqlx::query("INSERT INTO USUARIOS (NOME) VALUES($1);")
            .bind(data.nome)
            .execute(&**DB_CONNECTION_POOL)
            .await;
        result
    }

    async fn update(data: Self) -> Result<PgQueryResult, sqlx::Error> {
        let result = sqlx::query("UPDATE USUARIOS SET NOME = $1 WHERE ID = $2;")
            .bind(data.nome)
            .bind(data.id)
            .execute(&**DB_CONNECTION_POOL)
            .await;
        result
    }

    async fn delete(id: Self::IdType) -> Result<PgQueryResult, sqlx::Error> {
        let result = sqlx::query("DELETE FROM USUARIOS WHERE ID = $1;")
            .bind(id)
            .execute(&**DB_CONNECTION_POOL)
            .await;
        result
    }
}
