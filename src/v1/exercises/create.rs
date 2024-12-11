use chrono::NaiveDateTime;
use extractors::users::UserId;
use sqlx::query;
use structs::exercise_types::ExerciseType;
use uuid::Uuid;

use crate::*;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateExerciseBody {
    name: String,
    // todo: make this an enum
    exercise_type: ExerciseType,
    description: String,
    body_parts: Vec<String>,
    primary_muscles: Vec<String>,
    secondary_muscles: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type, PartialEq, sqlx::FromRow)]
pub struct Exercise {
    pub id: Uuid,
    pub name: String,
    pub exercise_type: ExerciseType,
    pub description: String,
    pub author_id: Uuid,
    pub official: bool,
    pub created_at: NaiveDateTime,
}

#[utoipa::path(post, path = "/create", responses((status = OK, body = String)), tag = super::EXERCISES_TAG)]
pub async fn create(
    State(state): State<AppState>,
    UserId(user): UserId,
    Json(body): Json<CreateExerciseBody>,
) -> Result<String, AppError> {
    let is_admin = query!("SELECT is_admin FROM users WHERE id = $1", user)
        .fetch_one(&*state.db)
        .await?
        .is_admin;

    query!(
        r#"
        INSERT INTO exercises (name, exercise_type, official, author_id, description)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        body.name,
        body.exercise_type as ExerciseType,
        is_admin,
        user,
        body.description
    )
    .fetch_one(&*state.db)
    .await?;

    todo!()
}
