use axum::extract::Query;
use serde::Deserialize;
use sqlx::{query_as, query_scalar};

use crate::extractors::users::UserId;
use crate::structs::body_weights::{BodyWeightEntry, BodyWeightListResponse};
use crate::structs::serde_utils::JSDate;
use crate::*;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListBodyWeightsQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
    pub from: Option<JSDate>,
    pub to: Option<JSDate>,
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    20
}

/// List user's body weight entries with pagination and optional date range
#[utoipa::path(
    get,
    path = "",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 20)"),
        ("from" = Option<String>, Query, description = "Filter from recorded_at (ISO 8601)"),
        ("to" = Option<String>, Query, description = "Filter to recorded_at (ISO 8601)")
    ),
    responses(
        (status = OK, body = BodyWeightListResponse),
        (status = UNAUTHORIZED, description = "Not authenticated")
    ),
    tag = super::BODY_WEIGHTS_TAG
)]
pub async fn list_body_weights(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Query(query): Query<ListBodyWeightsQuery>,
) -> Result<Json<BodyWeightListResponse>, AppError> {
    let offset = (query.page - 1) * query.per_page;
    let from = query.from.map(|date| date.0);
    let to = query.to.map(|date| date.0);

    let body_weights = query_as::<_, BodyWeightEntry>(
        r#"
        SELECT id, user_id, weight, weight_unit, recorded_at, created_at
        FROM user_body_weights
        WHERE user_id = $1
          AND ($2::timestamp IS NULL OR recorded_at >= $2)
          AND ($3::timestamp IS NULL OR recorded_at <= $3)
        ORDER BY recorded_at DESC
        LIMIT $4 OFFSET $5
        "#,
    )
    .bind(user_id)
    .bind(from)
    .bind(to)
    .bind(query.per_page)
    .bind(offset)
    .fetch_all(&*state.db)
    .await?;

    let total = query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM user_body_weights
        WHERE user_id = $1
          AND ($2::timestamp IS NULL OR recorded_at >= $2)
          AND ($3::timestamp IS NULL OR recorded_at <= $3)
        "#,
    )
    .bind(user_id)
    .bind(from)
    .bind(to)
    .fetch_one(&*state.db)
    .await?;

    Ok(Json(BodyWeightListResponse {
        body_weights,
        total,
        page: query.page,
        per_page: query.per_page,
    }))
}
