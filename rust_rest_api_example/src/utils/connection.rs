use rust_rest_api_example_data::sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use rust_rest_api_example_utils::config;

pub async fn get_connection() -> Pool<Postgres> {
    let connection_string = config::get_config("connection");

    PgPoolOptions::new()
        .connect(&connection_string)
        .await
        .unwrap()
}
