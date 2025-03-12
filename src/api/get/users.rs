use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use sqlx::{Pool, Sqlite};

use crate::{
    db::get_users_from_db,
    models::{User, UserWithoutPassword},
};

#[derive(Serialize)]
struct GetUsersResponse {
    users: Vec<UserWithoutPassword>,
}

pub async fn users(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match user.permissions.manage_users {
        true => {
            let users = get_users_from_db(db).await?;
            let users: Vec<UserWithoutPassword> = users
                .iter()
                .map(|u| UserWithoutPassword {
                    username: u.username.clone(),
                    terminate: u.terminate,
                    permissions: u.permissions.clone(),
                })
                .collect();
            Ok(Json(GetUsersResponse { users }))
        }
        false => Err((
            StatusCode::FORBIDDEN,
            "You don't have permission to fetch this resource".to_string(),
        )),
    }
}
