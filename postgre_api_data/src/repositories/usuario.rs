use crate::structs;
use crate::traits;
use crate::utils;

use sqlx::postgres::PgQueryResult;
use structs::table::Table;
use structs::usuario::Usuario;
use traits::crud::CRUD;

use tokio::runtime::Runtime;

impl CRUD<Usuario> for Table<Usuario> {
    type IdType = i64;
    type TableType = Usuario;

    fn get_all() -> Result<Vec<Self::TableType>, sqlx::Error> {
        let rt = Runtime::new().unwrap();
        let pool = utils::connection::get_connection();
        let result = rt.block_on(sqlx::query_as("SELECT * FROM USUARIOS;").fetch_all(&pool));
        rt.block_on(pool.close());
        result
    }

    fn get_by_id(id: Self::IdType) -> Result<Self::TableType, sqlx::Error> {
        let rt = Runtime::new().unwrap();
        let pool = utils::connection::get_connection();
        let result = rt.block_on(
            sqlx::query_as("SELECT * FROM USUARIOS WHERE ID = $1;")
                .bind(id)
                .fetch_one(&pool),
        );
        rt.block_on(pool.close());
        result
    }

    fn add(data: Self::TableType) -> Result<PgQueryResult, sqlx::Error> {
        let rt = Runtime::new().unwrap();
        let pool = utils::connection::get_connection();
        let result = rt.block_on(
            sqlx::query("INSERT INTO USUARIOS (NOME) VALUES($1);")
                .bind(data.nome)
                .execute(&pool),
        );
        rt.block_on(pool.close());
        result
    }

    fn update(data: Self::TableType) -> Result<PgQueryResult, sqlx::Error> {
        let rt = Runtime::new().unwrap();
        let pool = utils::connection::get_connection();
        let result = rt.block_on(
            sqlx::query("UPDATE USUARIOS SET NOME = $1 WHERE ID = $2;")
                .bind(data.nome)
                .bind(data.id)
                .execute(&pool),
        );
        rt.block_on(pool.close());
        result
    }

    fn remove(id: Self::IdType) -> Result<PgQueryResult, sqlx::Error> {
        let rt = Runtime::new().unwrap();
        let pool = utils::connection::get_connection();
        let result = rt.block_on(
            sqlx::query("DELETE FROM USUARIOS WHERE ID = $1;")
                .bind(id)
                .execute(&pool),
        );
        rt.block_on(pool.close());
        result
    }
}
