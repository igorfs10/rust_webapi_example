use rust_rest_api_example_data::sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use super::configuration::ConfigurationFile;

pub async fn get_connection() -> Pool<Postgres> {
    let connection_string = ConfigurationFile::get_configuration_file().connection;

    PgPoolOptions::new()
        .connect(&connection_string)
        .await
        .unwrap()
}
