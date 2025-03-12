use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use sqlx::{Pool, Sqlite};

use crate::{
    db::get_user_from_db,
    models::{User, UserWithoutPassword},
};

#[derive(Serialize)]
struct GetCurrentUserResponse {
    user: UserWithoutPassword,
}

pub async fn profile(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user = get_user_from_db(user.username, &db).await?;
    Ok(Json(GetCurrentUserResponse {
        user: UserWithoutPassword {
            username: user.username,
            terminate: user.terminate,
            permissions: user.permissions,
        },
    }))
}
