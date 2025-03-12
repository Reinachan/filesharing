use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::{
    db::get_user_from_db,
    models::{User, UserWithoutPassword},
};

#[derive(Deserialize)]
pub struct UserToGet {
    username: String,
}

#[derive(Serialize)]
pub struct Response {
    user: UserWithoutPassword,
}

pub async fn user(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Json(user_to_get): Json<UserToGet>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if user_to_get.username != user.username && !user.permissions.manage_users {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have the permissions to get information about other users".to_string(),
        ));
    }
    let user = get_user_from_db(user_to_get.username, &db).await?;
    Ok(Json(Response {
        user: UserWithoutPassword {
            username: user.username,
            terminate: user.terminate,
            permissions: user.permissions,
        },
    }))
}
