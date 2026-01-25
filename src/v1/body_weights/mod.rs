use crate::AppState;

pub(super) use super::*;

pub mod create;
pub mod delete;
pub mod list;
pub mod update;

pub const BODY_WEIGHTS_TAG: &str = "body_weights";

pub(super) fn router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list::list_body_weights))
        .routes(routes!(create::create_body_weight))
        .routes(routes!(update::update_body_weight))
        .routes(routes!(delete::delete_body_weight))
        .with_state(state)
}
