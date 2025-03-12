use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::{Pool, Sqlite};

use crate::{
    db::edit_user_permissions,
    models::{User, UserWithoutPassword},
};

pub async fn update_user_permissions(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Json(req_user): Json<UserWithoutPassword>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // disallow users without manage user priviledges to change a user's permissions
    if !user.permissions.manage_users {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have the permissions to change user permissions".to_string(),
        ));
    }

    edit_user_permissions(&db, &req_user).await?;

    Ok(Json(req_user))
}
