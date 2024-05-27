use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use hashbrown::HashMap;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    println!("Hello, world!");

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/request", post(request))
        .route("/login", post(login));

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
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

async fn login(
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<Option<LoginResponse>>) {
    let LoginRequest { username, password } = payload;

    if username != "admin" || password != "password" {
        return (StatusCode::UNAUTHORIZED, Json(None));
    }

    let claims = Claims {
        sub: username,
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
    };

    let secret = "my_secret";

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();

    let res = LoginResponse {
        access_token: token,
    };

    (StatusCode::OK, Json(Some(res)))
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
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