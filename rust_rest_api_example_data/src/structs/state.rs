use std::sync::Arc;

use sqlx::{Pool, Postgres};

pub struct State {
    pub db_pool: Arc<Pool<Postgres>>,
}

impl State {
    pub async fn new(pool: Pool<Postgres>) -> Self {
        State {
            db_pool: Arc::new(pool),
        }
    }
}
