pub(super) use utoipa_axum::router::OpenApiRouter;
pub(super) use utoipa_axum::routes;

use crate::AppState;

pub mod auth;
pub mod body_parts;
pub mod exercises;
pub mod muscles;

pub fn router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .with_state(state.clone())
        .nest("/auth", auth::router(state.clone()))
        .nest("/exercises", exercises::router(state.clone()))
        .nest("/body_parts", body_parts::router(state.clone()))
        .nest("/muscles", muscles::router(state.clone()))
}
