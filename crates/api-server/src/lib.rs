use serde::{Deserialize, Serialize};
use totp_rs::{Algorithm, Secret, TOTP};

pub mod api;
pub mod application;
pub mod domain;

pub async fn totp_create() -> TOTP {
    let secret = Secret::Encoded("OBWGC2LOFVZXI4TJNZTS243FMNZGK5BNGEZDG".to_string());

    TOTP::new(
        Algorithm::SHA256,
        6,
        1,
        30,
        secret.to_bytes().unwrap(),
    ).unwrap()
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct VerifyTOTPRequest {
    pub otp: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}