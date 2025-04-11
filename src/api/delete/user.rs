use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{Pool, Sqlite};

use crate::{db::delete_user_db, models::User};

pub async fn user(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = id.parse().unwrap_or(0);

    // disallow users without manage user priviledges from deleting another user
    if id != user.id && !user.permissions.manage_users {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have the permissions to delete other users".to_string(),
        ));
    }

    delete_user_db(&db, id).await
}
