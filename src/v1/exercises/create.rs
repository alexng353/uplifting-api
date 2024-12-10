use chrono::{DateTime, Utc};
use extractors::users::UserId;
use sqlx::{query, query_as};
use structs::exercise_types::ExerciseType;
use uuid::Uuid;

use crate::*;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateExerciseBody {
    name: String,
    // todo: make this an enum
    exercise_type: ExerciseType,
    description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type, PartialEq, sqlx::FromRow)]
pub struct Exercise {
    pub id: Uuid,
    pub name: String,
    pub exercise_type: ExerciseType,
    pub description: String,
    pub author_id: Uuid,
    pub official: bool,
    pub created_at: DateTime<Utc>,
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

    if !is_admin {
        return Err(AppError::Error(Errors::Unauthorized));
    }
    // INSERT INTO users (id, created_at, updated_at, role) VALUES ($1, NOW(), NOW(), ($2::text)::user_role) RETURNING id, created_at, updated_at, role as "role:UserRole"
    //
    let insert = query_as!(
        Exercise,
        r#"
        INSERT INTO exercises (name, exercise_type, description, author_id, official)
        VALUES ($1, ($2::text)::exercise_type, $3, $4, $5)
        RETURNING id, name, exercise_type, description, author_id, official, created_at
        "#,
        body.name,
        body.exercise_type.to_string(),
        body.description,
        user,
        false
    );

    todo!()
}
