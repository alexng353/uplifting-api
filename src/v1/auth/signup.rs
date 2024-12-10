use crate::*;
use sqlx::query;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SignupBody {
    real_name: String,
    username: String,
    email: String,
    password: String,
}

/// Sign up
#[utoipa::path(post, path = "/signup", responses((status = OK, body = String)), tag = super::AUTH_TAG)]
pub async fn signup(
    State(state): State<AppState>,
    Json(body): Json<SignupBody>,
) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Password hashing failed")
        .to_string();

    let user = query!(
        "INSERT INTO users (real_name, username, email, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, real_name, username, email, password_hash, is_admin, created_at",
        body.real_name,
        body.username,
        body.email,
        hash
    )
    .fetch_one(&*state.db)
    .await?;

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
