pub mod routes;
pub mod services;
pub mod models;
pub mod config;

use actix_cors::Cors;
use actix_web::{http::header, web::Data, App, HttpServer};
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
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::CONTENT_TYPE])
                    .max_age(3600),
            )
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
