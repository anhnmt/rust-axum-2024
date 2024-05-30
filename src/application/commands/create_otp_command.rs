use axum::http::StatusCode;
use axum::Json;
use crate::totp_create;

pub async fn create_otp_command() -> Result<Json<String>, StatusCode> {
    let totp = totp_create().await;
    let token = totp.generate_current().unwrap();

    Ok(Json(token))
}