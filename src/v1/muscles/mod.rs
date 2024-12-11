use crate::AppState;

pub(super) use super::*;

pub mod create;

pub const MUSCLES_TAG: &str = "muscles";

pub(super) fn router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create::create))
        .with_state(state)
}
