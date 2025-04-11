use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use sqlx::{Pool, Sqlite};

use crate::{
    db::get_user_from_db,
    models::{User, UserWithoutPassword},
};

#[derive(Serialize)]
pub struct Response {
    user: UserWithoutPassword,
}

pub async fn user(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = id.parse().unwrap_or(0);

    if id != user.id && !user.permissions.manage_users {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have the permissions to get information about other users".to_string(),
        ));
    }
    let user = get_user_from_db(id, &db).await?;
    Ok(Json(Response {
        user: UserWithoutPassword {
            id: user.id,
            username: user.username,
            terminate: user.terminate,
            permissions: user.permissions,
        },
    }))
}
