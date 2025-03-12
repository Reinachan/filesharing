use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::{Pool, Sqlite};

use crate::{db::create_user_db, models::User};

pub async fn create_user(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Json(new_user): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // disallow users without manage user priviledges from creating a new user
    if !user.permissions.manage_users {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have the permissions to create a user".to_string(),
        ));
    }

    create_user_db(&db, new_user).await
}
