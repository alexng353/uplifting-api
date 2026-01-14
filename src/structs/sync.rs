use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::sets::PreviousSetData;

/// Request to sync a completed workout from offline storage
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncWorkoutRequest {
    pub name: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub privacy: String,
    pub gym_location: Option<String>,
    pub exercises: Vec<SyncExercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncExercise {
    pub exercise_id: Uuid,
    pub profile_id: Option<Uuid>,
    pub sets: Vec<SyncSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncSet {
    pub reps: i32,
    pub weight: Decimal,
    pub weight_unit: String,
    pub created_at: NaiveDateTime,
}

/// Response after syncing a workout
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncWorkoutResponse {
    pub workout_id: Uuid,
    /// Updated previous sets data for exercises used in this workout
    pub previous_sets: Vec<PreviousSetData>,
}
