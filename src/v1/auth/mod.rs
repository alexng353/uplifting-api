use crate::state::AppState;

pub(super) use super::*;

pub mod signup;

pub const AUTH_TAG: &str = "auth";

pub fn router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(signup::signup))
        .with_state(state)
}
