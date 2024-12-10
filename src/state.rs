use std::sync::Arc;

use hmac::Hmac;
use sha2::Sha256;

#[derive(Clone)]
pub struct AppState {
    pub db: DB,
    // pub jwt_secret: String,
    pub jwt_key: Hmac<Sha256>,
}

pub type DB = Arc<sqlx::Pool<sqlx::Postgres>>;
