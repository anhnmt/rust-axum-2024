use axum::http::StatusCode;
use axum::Json;
use crate::{totp_create, VerifyTOTPRequest};

pub async fn verify_otp_command(
    Json(payload): Json<VerifyTOTPRequest>,
) -> StatusCode {
    let totp = totp_create().await;

    match totp.check_current(payload.otp.as_str()) {
        Ok(result) => {
            if result {
                StatusCode::OK
            } else {
                StatusCode::UNAUTHORIZED
            }
        },
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}