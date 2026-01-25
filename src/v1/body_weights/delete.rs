use axum::extract::Path;
use sqlx::{query, query_scalar};
use uuid::Uuid;

use crate::extractors::users::UserId;
use crate::*;

/// Delete a body weight entry
#[utoipa::path(
    delete,
    path = "/{entry_id}",
    params(
        ("entry_id" = Uuid, Path, description = "Body weight entry ID")
    ),
    responses(
        (status = OK, description = "Entry deleted"),
        (status = NOT_FOUND, description = "Entry not found"),
        (status = UNAUTHORIZED, description = "Not authenticated")
    ),
    tag = super::BODY_WEIGHTS_TAG
)]
pub async fn delete_body_weight(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(entry_id): Path<Uuid>,
) -> Result<&'static str, AppError> {
    let owner_id = query_scalar::<_, Uuid>(
        "SELECT user_id FROM user_body_weights WHERE id = $1",
    )
    .bind(entry_id)
    .fetch_optional(&*state.db)
    .await?;

    match owner_id {
        None => return Err(AppError::Error(Errors::Unauthorized)),
        Some(id) if id != user_id => return Err(AppError::Error(Errors::Unauthorized)),
        _ => {}
    }

    query("DELETE FROM user_body_weights WHERE id = $1")
        .bind(entry_id)
        .execute(&*state.db)
        .await?;

    Ok("deleted")
}
