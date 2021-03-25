use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::utils::connection::get_connection;

pub struct State {
    pub db_pool: Arc<Pool<Postgres>>,
}

impl State {
    pub async fn new() -> Self {
        State {
            db_pool: Arc::new(get_connection().await),
        }
    }
}
