use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash};
use sqlx::{Pool, Sqlite};

use crate::{
    db::edit_user_password,
    models::{User, UserIdPassword},
};

pub async fn update_password(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Json(req_user): Json<UserIdPassword>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // disallow users without manage user priviledges from editing another user's password
    if req_user.id != user.id && !user.permissions.manage_users {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have the permissions to change the password of other users".to_string(),
        ));
    }

    edit_user_password(
        &db,
        UserIdPassword {
            id: req_user.id,
            password: hash(req_user.password, DEFAULT_COST).unwrap(),
        },
    )
    .await
}
