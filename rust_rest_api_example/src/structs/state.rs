use std::sync::Arc;

use rust_rest_api_example_data::sqlx::{Pool, Postgres};

// Struct que vai ser com
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
