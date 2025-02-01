mod errors;
mod flows;
mod pages;
mod serve_authorization;
mod serve_login;
mod serve_tokens;
mod storage;

use crate::serve_authorization::serve_authorization;
use crate::serve_login::serve_login;
use crate::serve_tokens::serve_tokens;
use crate::storage::cache::Cache;
use crate::storage::database::Database;
use axum::routing::{get, post};
use axum::Router;
use dotenv::{dotenv, from_filename};
use log::info;
use std::env;
use tokio::sync::OnceCell;
use tokio_postgres::NoTls;

// Static global instances
static GLOBAL_CACHE: OnceCell<Cache> = OnceCell::const_new();
static GLOBAL_DATABASE: OnceCell<Database> = OnceCell::const_new();

async fn initialize_database() -> Result<(), Box<dyn std::error::Error>> {
    // Load database URL from environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the PostgresSQL database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // Spawn the connection handler
    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Database connection error: {}", err);
        }
    });

    // Initialize the global database
    let db = Database::new(client);
    GLOBAL_DATABASE
        .set(db)
        .expect("Global database should only be initialized once");

    Ok(())
}

fn initialize_cache() -> Result<(), Box<dyn std::error::Error>> {
    // Load Redis URL and namespace from environment
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let cache_namespace = env::var("CACHE_NAMESPACE").unwrap_or_else(|_| "default".to_string());

    // Connect to Redis
    let client = redis::Client::open(redis_url)?;
    GLOBAL_CACHE
        .set(Cache::new(client, cache_namespace))
        .expect("Global cache should only be initialized once");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok(); // Load from .env by default
    let env_file = env::var("ENV_FILE").unwrap_or_else(|_| ".env".to_string());
    from_filename(env_file).ok();
    env_logger::init();

    // Initialize the database and cache
    initialize_database().await?;
    initialize_cache()?;

    info!("Initialization complete!");
    info!("Starting the server!");

    let app = Router::new()
        .route("/", get(root))
        .route("/authorize", get(serve_authorization))
        .route("/login", post(serve_login))
        .route("/token", post(serve_tokens));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
