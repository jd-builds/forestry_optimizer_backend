use chrono::Local;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use std::io::Write;

mod api;
mod config;
mod db;
mod docs;
mod errors;
mod models;
mod repositories;
mod routes;
mod schema;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = load_config()?;
    initialize_logger(&config);
    let _guard = initialize_sentry(&config);
    let pool = create_db_pool(&config)?;

    info!("Starting application in {:?} mode", config.environment);

    server::run(config, pool).await
}

fn load_config() -> std::io::Result<config::Config> {
    config::Config::load().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })
}

fn initialize_logger(config: &config::Config) {
    env_logger::Builder::from_env(Env::default().default_filter_or(&config.log_level))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

fn initialize_sentry(config: &config::Config) -> Option<sentry::ClientInitGuard> {
    config.sentry_dsn.clone().map(|sentry_dsn| {
        sentry::init((
            sentry_dsn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                environment: Some(config.environment.to_string().into()),
                ..Default::default()
            },
        ))
    })
}

fn create_db_pool(
    config: &config::Config,
) -> std::io::Result<Pool<ConnectionManager<PgConnection>>> {
    db::create_connection_pool(&config.database_url).map_err(|e| {
        error!("Failed to create database pool: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })
}
