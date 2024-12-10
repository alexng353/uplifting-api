use axum::{extract::State, response::Response};

use crate::state::AppState;

use super::*;

/// Sign up
#[utoipa::path(get, path = "/signup", responses((status = OK, body = String)), tag = super::AUTH_TAG)]
pub async fn signup(State(state): State<AppState>) -> Response<String> {
    Response::new("Hello, World!".to_string())
}
