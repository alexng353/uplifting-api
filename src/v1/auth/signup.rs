use crate::*;
use sqlx::query;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SignupBody {
    real_name: String,
    username: String,
    email: String,
    password: String,
}

/// Sign up
#[utoipa::path(get, path = "/signup", responses((status = OK, body = String)), tag = super::AUTH_TAG)]
pub async fn signup(
    State(state): State<AppState>,
    Json(body): Json<SignupBody>,
) -> Result<String, AppError> {
    let user = query!(
        "INSERT INTO users (real_name, username, email, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, real_name, username, email, password_hash, is_admin, created_at",
        body.real_name,
        body.username,
        body.email,
        body.password,
    )
    .fetch_one(&*state.db)
    .await?;

    Ok(user.id.to_string())
}
