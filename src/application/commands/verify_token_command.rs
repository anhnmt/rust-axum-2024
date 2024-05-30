use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::Claims;

pub async fn verify_token_command(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(token) = header_map.get("Authorization") {
        let token = token.to_str().unwrap().replace("Bearer ", "");

        return match decode::<Claims>(
            &token,
            &DecodingKey::from_secret("my_secret".as_ref()),
            &Validation::default(),
        ) {
            Ok(token_data) => Ok(Json(token_data.claims.sub)),
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        };
    }

    Err(StatusCode::UNAUTHORIZED)
}