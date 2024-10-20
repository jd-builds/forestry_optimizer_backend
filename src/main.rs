use dotenv::dotenv;

mod schema;
mod config;
mod db;
mod routes;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::load_config();
    let pool = db::create_connection_pool(&config.database_url);

    println!("Starting server on {}:{}", config.host, config.port);
    server::run(config, pool).await
}
