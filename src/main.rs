use std::collections::HashMap;
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("Hello, world!");

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/request", post(request));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

async fn request() -> (StatusCode, Json<HashMap<String, String>>) {
    let res = reqwest::get("https://ipinfo.io/json")
        .await.unwrap()
        .json::<HashMap<String, String>>()
        .await.unwrap();

    (StatusCode::CREATED, Json(res))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}