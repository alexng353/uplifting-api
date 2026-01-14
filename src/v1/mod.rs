pub(super) use utoipa_axum::router::OpenApiRouter;
pub(super) use utoipa_axum::routes;

use crate::AppState;

pub mod auth;
pub mod exercises;
pub mod muscles;
pub mod workouts;

pub fn router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .with_state(state.clone())
        .nest("/auth", auth::router(state.clone()))
        .nest("/exercises", exercises::router(state.clone()))
        .nest("/muscles", muscles::router(state.clone()))
        .nest("/workouts", workouts::router(state.clone()))
}
