use axum::http::StatusCode;
use axum::Json;
use hashbrown::HashMap;

pub async fn get_ipinfo_query() -> (StatusCode, Json<HashMap<String, String>>) {
    let res = reqwest::get("https://ipinfo.io/json")
        .await.unwrap()
        .json::<HashMap<String, String>>()
        .await.unwrap();

    (StatusCode::CREATED, Json(res))
}