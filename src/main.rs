mod api;
mod auth;
mod constants;
mod db;
mod handlers;
mod helpers;
mod models;
mod routes;
mod tasks;
mod views;

use constants::{FEATURES, ROOT_FOLDER, SERVER_DOMAIN, SERVER_PORT};
use routes::{delete_file_route, download_file, get_file, upload_file};
use tasks::scheduled_deletion;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use views::{all_files, root, upload};

use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware,
    routing::{get, post, put},
};
use sqlx::SqlitePool;
use std::{fs::create_dir, path, time::Duration};

use crate::{
    api::{delete, get, post, put},
    auth::{authorization_middleware, request_token},
    routes::{auth, create_user, delete_user, edit_user, put_upload_file},
    tasks::create_default_user,
    views::{all_users, new_user, profile, sign_in},
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Couldn't find .env file");

    // Create files and folders
    if !path::Path::new(&*ROOT_FOLDER).exists() {
        create_dir(path::Path::new(&*ROOT_FOLDER)).expect("couldn't create folder 'files'");
    }
    if !path::Path::new("db").exists() {
        create_dir(path::Path::new("db")).expect("couldn't create folder 'db'");
    }
    if !path::Path::new("db/files.sqlite").exists() {
        std::fs::File::create("db/files.sqlite").unwrap();
    }

    // Create database pool
    let conn = SqlitePool::connect("sqlite://db/files.sqlite")
        .await
        .expect("No DB Pool");

    // Run migrations
    sqlx::migrate!("./migrations").run(&conn).await.unwrap();

    create_default_user(conn.clone()).await;

    tokio::spawn(async {
        let conn2 = SqlitePool::connect("sqlite://db/files.sqlite")
            .await
            .expect("No DB Pool");

        loop {
            match scheduled_deletion(conn2.clone()).await {
                Ok(_) => {}
                Err(err) => println!("{err:#?}"),
            };
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });

    let views_routes = Router::new()
        .route("/", get(root).post(download_file))
        .route("/signin", get(sign_in))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/upload", get(upload))
        .route("/profile", get(profile))
        .route("/delete", post(delete_file_route))
        .route("/files", get(all_files))
        .route("/user", get(new_user).post(create_user))
        .route("/users", get(all_users))
        .route("/delete-user", post(delete_user))
        .route("/edit-user", post(edit_user))
        .route("/{file_name}", get(get_file));

    let api_routes = Router::new()
        .route("/profile", get(get::profile))
        .route("/user", post(post::create_user))
        .route("/user/{username}", get(get::user).delete(delete::user))
        .route("/user/permissions", put(put::update_user_permissions))
        .route("/user/password", put(put::update_password))
        .route("/users", get(get::users))
        .route(
            "/admin/files",
            get(get::admin_files).delete(delete::admin_files),
        )
        .layer(middleware::from_fn_with_state(
            conn.clone(),
            authorization_middleware,
        ))
        .route("/token", post(request_token));

    // Set up routes
    let mut app = Router::new();

    // handle custom client
    if FEATURES.contains(&"custom_client") {
        if !FEATURES.contains(&"disable_default_client") {
            app = app.nest("/legacy", views_routes);
        }
    } else {
        app = app.merge(views_routes);
    }

    let app = app
        .nest("/api", api_routes)
        .route("/auth", post(auth))
        .route("/upload", post(upload_file).put(put_upload_file))
        .with_state(conn)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024 * 20));
    //                               ^ sets max filesize to 20 GB

    println!("Server domain is set to {}", *SERVER_DOMAIN);
    println!("Starting server at http://localhost:{}", *SERVER_PORT);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", *SERVER_PORT))
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
