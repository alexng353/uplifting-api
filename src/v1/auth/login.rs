use crate::*;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use sqlx::query;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginBody {
    username: String,
    password: String,
}

/// Login
#[utoipa::path(post, path = "/login", responses((status = OK, body = String)), tag = super::AUTH_TAG)]
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginBody>,
) -> Result<String, AppError> {
    let user = query!(
        "SELECT id, real_name, username, email, password_hash, is_admin, created_at
        FROM users
        WHERE username = $1",
        body.username,
    )
    .fetch_one(&*state.db)
    .await?;

    let argon2 = Argon2::default();
    let hash = PasswordHash::new(&body.password).expect("Password hashing failed");

    if !argon2
        .verify_password(body.password.as_bytes(), &hash)
        .is_ok()
    {
        return Err(AppError::Error(Errors::Unauthorized));
    }

    // technically I should be using a JWT and returning it to the client
    // since this will be a mobile app.

    // let jwt = ...

    Ok(user.id.to_string())
}
