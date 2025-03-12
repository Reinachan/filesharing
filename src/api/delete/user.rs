use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use sqlx::{Pool, Sqlite};

use crate::{db::delete_user_db, models::User};

#[derive(Deserialize)]
pub struct UserToDelete {
    username: String,
}

pub async fn user(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Json(user_to_delete): Json<UserToDelete>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // disallow users without manage user priviledges from deleting another user
    if user_to_delete.username != user.username && !user.permissions.manage_users {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have the permissions to delete other users".to_string(),
        ));
    }
    delete_user_db(&db, user_to_delete.username).await?;

    Ok(StatusCode::NO_CONTENT)
}
