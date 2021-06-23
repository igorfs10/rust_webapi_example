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

// pub fn run_sync<T, U>(future: impl Future<Output = T>)-> U {
//     let rt = tokio::runtime::Runtime::new().unwrap();

//     let ret = rt.block_on(async{future})?;

//     ret
// }
