// src/db/pool.rs
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
//Pgpool- A pool of PostgreSQL connections
// PgPoolOptions - The "configuration options" for creating a pool (the max number of connections).
pub async fn create_pool() -> PgPool {
    // Retrieve the database URL from an environment variable
    let database_url = env::var("DATABASE_URL")
    //"Panic" if no url is found
        .expect("DATABASE_URL must be set in .env or environment");

    // Creates a connection pool with up to 5 connections
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool")
}

// Function which can be optionally called with CLI arguments
// This function will run the migrations
pub async fn migrate_db(pool: &PgPool) -> bool {
    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => true,
        Err(e) => {
            println!("Failed to migrate the database: {}", e);
            false
        }
    }
}





