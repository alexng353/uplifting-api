use crate::AppState;

pub(super) use super::*;

pub mod create;

pub const BODY_PARTS_TAG: &str = "body_parts";

pub(super) fn router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create::create))
        .with_state(state)
}
