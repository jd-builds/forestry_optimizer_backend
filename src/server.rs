use actix_web::{web, App, HttpServer, middleware::Logger};
use tracing::info;
use env_logger;

use crate::application::{config::AppConfig, AppState};
use crate::application::api::routes;

pub async fn run(config: AppConfig, state: AppState) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let app_state = web::Data::new(state);
    
    let server = HttpServer::new(move || {
        info!("Configuring application");
        App::new()
            .wrap(Logger::new("%r %s %b %D ms"))
            .app_data(app_state.clone())
            .configure(routes::configure)
    })
    .bind((config.host.clone(), config.port))?;

    info!("Server running at http://{}:{}", config.host, config.port);
    server.run().await
}