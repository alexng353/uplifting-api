pub struct User {
    pub id: uuid::Uuid,
    pub real_name: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: chrono::NaiveDateTime,
}
