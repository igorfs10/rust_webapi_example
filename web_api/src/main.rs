mod controllers;
mod structs;

use actix_web::{App, HttpServer};
use utils::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = config::get_config("server_address");

    println!("Running the server at {}", server_address);
    HttpServer::new(move || App::new().configure(controllers::usuario::init))
        .bind(&server_address)?
        .run()
        .await
}
