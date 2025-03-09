use axum::http::StatusCode;
use axum_extra::headers::Cookie;
use bcrypt::verify;
use http_auth_basic::Credentials;
use lazy_static::lazy_static;
use sqlx::{Pool, Sqlite};

use crate::{
    constants::AUTH_COOKIE,
    models::{Permissions, PermissionsDB, User, UserDB},
};

lazy_static! {
    pub static ref LACKING_PERMISSION: (StatusCode, String) = (
        StatusCode::FORBIDDEN,
        "Lacking required permissions".to_string(),
    );
}

pub enum AuthOrBasic {
    Cookie(Cookie),
    Basic((String, String)),
}

pub async fn check_auth(
    db: &Pool<Sqlite>,
    auth: AuthOrBasic,
    required_permissions: Option<Permissions>,
) -> Result<User, (StatusCode, String)> {
    let (username, password) = match auth {
        AuthOrBasic::Cookie(value) => {
            let auth_cookie = value.get(AUTH_COOKIE).unwrap_or("").to_string();
            match Credentials::from_header(auth_cookie) {
                Ok(value) => (value.user_id, value.password),
                Err(_) => return Err((StatusCode::FORBIDDEN, "Try signing in again".to_string())),
            }
        }
        AuthOrBasic::Basic(value) => value,
    };

    let user = sqlx::query_as!(
        UserDB,
        "
        SELECT * FROM users WHERE username = ?
        ",
        username
    )
    .fetch_one(db)
    .await
    .map_err(|_| (StatusCode::FORBIDDEN, "Permission Denied".to_string()))?;

    let verified = verify(password, &user.password)
        .map_err(|_| (StatusCode::FORBIDDEN, "Wrong password".to_string()))?;

    if !verified {
        return Err((StatusCode::FORBIDDEN, "Wrong password".to_string()));
    }

    let permissions = sqlx::query_as!(
        PermissionsDB,
        "
        SELECT * FROM permissions WHERE username = ?
        ",
        username
    )
    .fetch_one(db)
    .await
    .map_err(|_| {
        (
            StatusCode::FORBIDDEN,
            "Not signed in, try again".to_string(),
        )
    })?;

    if required_permissions.is_some() {
        let perm = required_permissions.unwrap();
        if perm.manage_users && !permissions.manage_users {
            return Err(LACKING_PERMISSION.to_owned());
        }

        if perm.delete_files && !permissions.delete_files {
            return Err(LACKING_PERMISSION.to_owned());
        }

        if perm.list_files && !permissions.list_files {
            return Err(LACKING_PERMISSION.to_owned());
        }

        if perm.upload_files && !permissions.upload_files {
            return Err(LACKING_PERMISSION.to_owned());
        }
    }

    Ok(User {
        username: user.username,
        password: user.password,
        permissions: Permissions {
            manage_users: permissions.manage_users,
            upload_files: permissions.upload_files,
            list_files: permissions.list_files,
            delete_files: permissions.delete_files,
        },
        terminate: user.terminate,
    })
}
