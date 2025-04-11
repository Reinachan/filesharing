use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::{Pool, Sqlite};

use crate::{
    db::edit_user_username,
    models::{User, UserNameID},
};

pub async fn update_username(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Json(req_user): Json<UserNameID>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // disallow users without manage user priviledges from editing another user's password
    if req_user.id != user.id && !user.permissions.manage_users {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have the permissions to change the username of other users".to_string(),
        ));
    }

    edit_user_username(&db, req_user).await
}
