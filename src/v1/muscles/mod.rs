use crate::AppState;

pub(super) use super::*;

pub mod create;
pub mod get_all;

pub const MUSCLES_TAG: &str = "muscles";

pub(super) fn router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create::create))
        .routes(routes!(get_all::get_all))
        .with_state(state)
}
