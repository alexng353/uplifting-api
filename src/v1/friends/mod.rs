use crate::AppState;

pub(super) use super::*;

pub mod feed;
pub mod list;
pub mod remove;
pub mod requests;
pub mod respond;
pub mod send;

pub const FRIENDS_TAG: &str = "friends";

pub(super) fn router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list::list_friends))
        .routes(routes!(requests::list_requests))
        .routes(routes!(send::send_request))
        .routes(routes!(respond::respond_request))
        .routes(routes!(remove::remove_friend))
        .routes(routes!(feed::get_feed))
        .with_state(state)
}
