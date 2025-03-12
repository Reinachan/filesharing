use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::{Pool, Sqlite};

use crate::{
    db::edit_user_password,
    models::{User, UsernamePassword},
};

pub async fn update_password(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Json(req_user): Json<UsernamePassword>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // disallow users without manage user priviledges from editing another user's password
    if req_user.username != user.username && !user.permissions.manage_users {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have the permissions to change the password of other users".to_string(),
        ));
    }

    edit_user_password(&db, req_user).await
}
