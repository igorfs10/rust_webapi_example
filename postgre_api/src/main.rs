mod controllers;

use actix_web::{App, HttpServer};
use postgre_api_data::utils::configuration::ConfigurationFile;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = ConfigurationFile::get_configuration_file().server_address;
    println!("Starting the server at {}", server_address);
    HttpServer::new(|| App::new().configure(controllers::usuario::init))
        .bind(&server_address)?
        .run()
        .await
}
