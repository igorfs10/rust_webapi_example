mod controllers;
mod structs;
mod utils;

use actix_web::{web, App, HttpServer};
use rust_rest_api_example_utils::config;
use structs::state::State;
use utils::connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = config::get_config("server_address");

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
