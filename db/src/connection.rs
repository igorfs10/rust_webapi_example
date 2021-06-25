use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use utils::config;

pub fn get_db_connection_pool() -> Pool<Postgres> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        PgPoolOptions::new()
            .connect(&config::get_config("connection"))
            .await
            .unwrap()
    })
}
