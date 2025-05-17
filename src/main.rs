// Arquivo: src/main.rs

mod db;
mod auth;
mod models;
mod routes;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;
use db::init_db_pool;
use routes::users::user_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_db_pool(&database_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .configure(user_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}