mod controllers;

use actix_web::{web, App, HttpServer};
use postgre_api_data::{structs::state::State, utils::configuration::ConfigurationFile};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = ConfigurationFile::get_configuration_file().server_address;

    let app_state = web::Data::new(State::new().await);

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
