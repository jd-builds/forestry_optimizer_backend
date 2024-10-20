use actix_web::{web, App, HttpServer, HttpResponse, Responder, get};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

// mod models;
// mod repositories;
// mod api;
mod schema;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
