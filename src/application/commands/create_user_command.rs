use axum::http::StatusCode;
use axum::Json;
use crate::{CreateUser, User};

pub async fn create_user_command(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}