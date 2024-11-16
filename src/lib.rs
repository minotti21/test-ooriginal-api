pub mod routes;
pub mod services;
pub mod models;
pub mod config;

use actix_web::{web::Data, App, HttpServer};
use config::get_database_pool;
use sqlx::{Pool, Postgres};
use dotenv::dotenv;

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn setup_server() -> std::io::Result<()> {
    dotenv().ok();

    let pool = get_database_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
