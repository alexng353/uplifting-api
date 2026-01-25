use sqlx::query_as;

use crate::extractors::users::UserId;
use crate::structs::body_weights::{BodyWeightEntry, CreateBodyWeightEntry};
use crate::*;

/// Create a new body weight entry
#[utoipa::path(
    post,
    path = "",
    request_body = CreateBodyWeightEntry,
    responses(
        (status = OK, body = BodyWeightEntry),
        (status = UNAUTHORIZED, description = "Not authenticated")
    ),
    tag = super::BODY_WEIGHTS_TAG
)]
pub async fn create_body_weight(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Json(body): Json<CreateBodyWeightEntry>,
) -> Result<Json<BodyWeightEntry>, AppError> {
    let recorded_at = body.recorded_at.map(|date| date.0);

    let entry = query_as::<_, BodyWeightEntry>(
        r#"
        INSERT INTO user_body_weights (user_id, weight, weight_unit, recorded_at)
        VALUES ($1, $2, COALESCE($3, 'kg'), COALESCE($4, NOW()))
        RETURNING id, user_id, weight, weight_unit, recorded_at, created_at
        "#,
    )
    .bind(user_id)
    .bind(body.weight)
    .bind(body.weight_unit)
    .bind(recorded_at)
    .fetch_one(&*state.db)
    .await?;

    Ok(Json(entry))
}
