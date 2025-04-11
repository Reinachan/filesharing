use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash};
use sqlx::{Pool, Sqlite};

use crate::{
    db::create_user_db,
    models::{CreateUserDB, User, UserWithoutPassword},
};

#[axum::debug_handler]
pub async fn create_user(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Json(new_user): Json<CreateUserDB>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // disallow users without manage user priviledges from creating a new user
    if !user.permissions.manage_users {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have the permissions to create a user".to_string(),
        ));
    }

    let created_user = create_user_db(
        &db,
        CreateUserDB {
            username: new_user.username.clone(),
            password: hash(new_user.password, DEFAULT_COST).unwrap(),
            terminate: new_user.terminate,
            permissions: new_user.permissions,
        },
    )
    .await?;

    Ok(Json(UserWithoutPassword {
        id: created_user.id,
        username: created_user.username,
        terminate: created_user.terminate,
        permissions: created_user.permissions,
    }))
}
