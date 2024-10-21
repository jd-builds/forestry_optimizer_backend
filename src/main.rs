use dotenv::dotenv;
use std::io;

mod schema;
mod config;
mod db;
mod routes;
mod server;
mod models;
mod repositories;
mod api;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = config::load_config();
    let pool = db::create_connection_pool(&config.database_url);

    println!("Starting server on {}:{}", config.host, config.port);
    
    server::run(config, pool).await
}
