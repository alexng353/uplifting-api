use crate::*;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use sqlx::query;
use std::collections::BTreeMap;

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

    let key: Hmac<Sha256> =
        Hmac::new_from_slice(state.jwt_secret.as_bytes()).context("Failed to create HMAC")?;
    let mut claims = BTreeMap::new();
    claims.insert("id", user.id.to_string());
    claims.insert("username", user.username);
    claims.insert("real_name", user.real_name);
    claims.insert("email", user.email);
    let token_str = claims.sign_with_key(&key).context("Failed to sign JWT")?;

    Ok(token_str)
}
