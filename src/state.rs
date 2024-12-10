use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DB,
    pub jwt_secret: String,
}

pub type DB = Arc<sqlx::Pool<sqlx::Postgres>>;
