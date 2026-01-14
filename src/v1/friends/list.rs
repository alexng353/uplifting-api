use sqlx::query_as;

use crate::extractors::users::UserId;
use crate::structs::friends::FriendWithProfile;
use crate::*;

/// List accepted friends
#[utoipa::path(
    get,
    path = "",
    responses(
        (status = OK, body = Vec<FriendWithProfile>),
        (status = UNAUTHORIZED, description = "Not authenticated")
    ),
    tag = super::FRIENDS_TAG
)]
pub async fn list_friends(
    State(state): State<AppState>,
    UserId(user_id): UserId,
) -> Result<Json<Vec<FriendWithProfile>>, AppError> {
    let friends = query_as!(
        FriendWithProfile,
        r#"
        SELECT * FROM (
            SELECT 
                f.id as "friendship_id!",
                u.id as "user_id!",
                u.username as "username!",
                u.real_name as "real_name!",
                u.avatar_url,
                f.status as "status!",
                f.created_at as "created_at!"
            FROM friendships f
            JOIN users u ON u.id = f.friend_id
            WHERE f.user_id = $1 AND f.status = 'accepted'
            UNION
            SELECT 
                f.id as "friendship_id!",
                u.id as "user_id!",
                u.username as "username!",
                u.real_name as "real_name!",
                u.avatar_url,
                f.status as "status!",
                f.created_at as "created_at!"
            FROM friendships f
            JOIN users u ON u.id = f.user_id
            WHERE f.friend_id = $1 AND f.status = 'accepted'
        ) friends
        ORDER BY "created_at!" DESC
        "#,
        user_id
    )
    .fetch_all(&*state.db)
    .await?;

    Ok(Json(friends))
}
