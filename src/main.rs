mod cache;
mod database;
mod errors;
mod flows;
mod pages;
mod serve_authorization;
mod serve_login;
mod storage;

use crate::serve_authorization::serve_authorization;
use crate::serve_login::serve_login;
use axum::routing::post;
use axum::{routing::get, Router};
use std::sync::OnceLock;

static REDIS_CLIENT: OnceLock<redis::Client> = OnceLock::new();
static POSTGRES_CLIENT: OnceLock<tokio_postgres::Client> = OnceLock::new();

#[tokio::main]
async fn main() {
    initialize_redis_client();
    initialize_postgres_client().await;

    println!("Starting the server!");

    let app = Router::new()
        .route("/", get(root))
        .route("/authorize", get(serve_authorization))
        .route("/login", post(serve_login));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

fn initialize_redis_client() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap_or_else(|err| {
        panic!("{}", err);
    });

    REDIS_CLIENT.set(client).unwrap();
}

pub fn get_redis_connection() -> redis::Connection {
    REDIS_CLIENT
        .get()
        .unwrap()
        .get_connection()
        .unwrap_or_else(|err| {
            panic!("{}", err);
        })
}

async fn initialize_postgres_client() {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres",
        postgres::NoTls,
    )
    .await
    .unwrap_or_else(|err| {
        panic!("{}", err);
    });

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    POSTGRES_CLIENT.set(client).unwrap();
}
