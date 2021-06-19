pub mod repositories;
pub mod structs;
pub mod traits;

mod connection;

pub use serde;
pub use sqlx;

use sqlx::Pool;
use sqlx::Postgres;
use static_init::dynamic;
use std::sync::Arc;

#[dynamic]
static POOL: Arc<Pool<Postgres>> = Arc::new(connection::get_pool());
