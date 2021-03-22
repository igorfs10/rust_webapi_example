use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::runtime::Runtime;

use super::configuration::ConfigurationFile;

pub fn get_connection() -> Pool<Postgres> {
    let connection_string = ConfigurationFile::get_configuration_file().connection;

    let rt = Runtime::new().unwrap();
    rt.block_on(PgPoolOptions::new().connect(&connection_string))
        .unwrap()
}
