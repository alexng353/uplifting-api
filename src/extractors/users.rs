use std::sync::Arc;

use crate::*;
use anyhow::bail;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts},
};

use chrono::{DateTime, Utc};
use jwt::VerifyWithKey;
use sqlx::types::Uuid;
use util::auth::JWTClaims;

pub struct UserId(Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for UserId
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, s: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(s);

        let jwt_string = parts
            .headers
            .get(header::AUTHORIZATION)
            .ok_or(AppError::Error(Errors::Unauthorized))?
            .to_str()
            .map_err(|_| AppError::Error(Errors::Unauthorized))?
            .strip_prefix("Bearer ")
            .ok_or(AppError::Error(Errors::Unauthorized))?;

        let claims: JWTClaims = jwt_string
            .verify_with_key(&state.jwt_key)
            .map_err(|_| AppError::Error(Errors::Unauthorized))?;

        Ok(UserId(claims.sub))
    }
}
