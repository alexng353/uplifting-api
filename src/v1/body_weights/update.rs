use axum::extract::Path;
use sqlx::{query_as, query_scalar};
use uuid::Uuid;

use crate::extractors::users::UserId;
use crate::structs::body_weights::{BodyWeightEntry, UpdateBodyWeightEntry};
use crate::*;

/// Update a body weight entry
#[utoipa::path(
    put,
    path = "/{entry_id}",
    params(
        ("entry_id" = Uuid, Path, description = "Body weight entry ID")
    ),
    request_body = UpdateBodyWeightEntry,
    responses(
        (status = OK, body = BodyWeightEntry),
        (status = NOT_FOUND, description = "Entry not found"),
        (status = UNAUTHORIZED, description = "Not authenticated")
    ),
    tag = super::BODY_WEIGHTS_TAG
)]
pub async fn update_body_weight(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(entry_id): Path<Uuid>,
    Json(body): Json<UpdateBodyWeightEntry>,
) -> Result<Json<BodyWeightEntry>, AppError> {
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

    let recorded_at = body.recorded_at.map(|date| date.0);

    let entry = query_as::<_, BodyWeightEntry>(
        r#"
        UPDATE user_body_weights
        SET 
            weight = COALESCE($2, weight),
            weight_unit = COALESCE($3, weight_unit),
            recorded_at = COALESCE($4, recorded_at)
        WHERE id = $1
        RETURNING id, user_id, weight, weight_unit, recorded_at, created_at
        "#,
    )
    .bind(entry_id)
    .bind(body.weight)
    .bind(body.weight_unit)
    .bind(recorded_at)
    .fetch_one(&*state.db)
    .await?;

    Ok(Json(entry))
}
