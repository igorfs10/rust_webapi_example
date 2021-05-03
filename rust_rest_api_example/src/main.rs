mod controllers;
mod utils;
mod structs;

use actix_web::{web, App, HttpServer};
use structs::state::State;
use utils::configuration::ConfigurationFile;
use utils::connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = ConfigurationFile::get_configuration_file().server_address;

    let app_state = web::Data::new(State::new(connection::get_connection().await).await);

    println!("Running the server at {}", server_address);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(controllers::usuario::init)
    })
    .bind(&server_address)?
    .run()
    .await
}
