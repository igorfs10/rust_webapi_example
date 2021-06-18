pub mod repositories;
pub mod structs;
pub mod traits;

pub use serde;
pub use sqlx;

use rust_rest_api_example_utils::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use static_init::dynamic;
use std::sync::Arc;

#[dynamic]
static POOL: Arc<Pool<Postgres>> = Arc::new(get_pool());

fn get_pool() -> Pool<Postgres> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        PgPoolOptions::new()
            .connect(&config::get_config("connection"))
            .await
            .unwrap()
    })
}
