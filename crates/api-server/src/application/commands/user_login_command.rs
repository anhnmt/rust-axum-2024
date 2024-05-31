use axum::http::StatusCode;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::{Claims, LoginRequest, LoginResponse};


pub async fn user_login_command(
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<Option<LoginResponse>>) {
    let LoginRequest { username, password } = payload;

    if username != "admin" || password != "password" {
        return (StatusCode::UNAUTHORIZED, Json(None));
    }

    let claims = Claims {
        sub: username,
        exp: (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("my_secret".as_ref()),
    ).unwrap();

    let res = LoginResponse {
        access_token: token,
    };

    (StatusCode::OK, Json(Some(res)))
}