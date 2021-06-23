pub mod repos;
pub mod structs;
pub mod traits;

mod connection;

pub use async_trait;
pub use sqlx;

use sqlx::Pool;
use sqlx::Postgres;
use static_init::dynamic;
use std::sync::Arc;

#[dynamic]
static DB_CONNECTION_POOL: Arc<Pool<Postgres>> = Arc::new(connection::get_db_connection_pool());
