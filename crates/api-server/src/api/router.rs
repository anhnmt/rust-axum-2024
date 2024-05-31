use axum::Router;
use axum::routing::{get, post};

use crate::application::commands::create_otp_command::create_otp_command;
use crate::application::commands::create_user_command::create_user_command;
use crate::application::commands::user_login_command::user_login_command;
use crate::application::commands::verify_otp_command::verify_otp_command;
use crate::application::commands::verify_token_command::verify_token_command;
use crate::application::queries::get_ipinfo_query::get_ipinfo_query;
use crate::application::queries::hello_world_query::hello_world_query;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello_world_query))

        .route("/users", post(create_user_command))
        .route("/login", post(user_login_command))
        .route("/token", post(verify_token_command))

        .route("/ipinfo", get(get_ipinfo_query))

        .route("/otp", post(create_otp_command))
        .route("/verify-otp", post(verify_otp_command))
}