use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn get_database_pool() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Error building a connection pool")
}