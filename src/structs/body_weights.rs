use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use super::serde_utils::JSDate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BodyWeightEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub weight: Decimal,
    pub weight_unit: String,
    pub recorded_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateBodyWeightEntry {
    pub weight: Decimal,
    pub weight_unit: Option<String>,
    pub recorded_at: Option<JSDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateBodyWeightEntry {
    pub weight: Option<Decimal>,
    pub weight_unit: Option<String>,
    pub recorded_at: Option<JSDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BodyWeightListResponse {
    pub body_weights: Vec<BodyWeightEntry>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
