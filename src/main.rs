mod constants;
mod db;
mod handlers;
mod helpers;
mod models;
mod routes;
mod tasks;
mod views;

use constants::ROOT_FOLDER;
use routes::{delete_file_route, download_file, get_file, upload_file};
use tasks::scheduled_deletion;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use views::{all_files, root, upload};

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use sqlx::SqlitePool;
use std::{fs::create_dir, path, time::Duration};

use crate::{
    constants::SERVER_DOMAIN,
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

    // Set up routes
    let app = Router::new()
        .route("/", get(root).post(download_file))
        .route("/signin", get(sign_in))
        .route("/auth", post(auth))
        .nest_service("/assets", ServeDir::new("assets"))
        .route(
            "/upload",
            get(upload).post(upload_file).put(put_upload_file),
        )
        .route("/profile", get(profile))
        .route("/delete", post(delete_file_route))
        .route("/files", get(all_files))
        .route("/user", get(new_user).post(create_user))
        .route("/users", get(all_users))
        .route("/delete-user", post(delete_user))
        .route("/edit-user", post(edit_user))
        .route("/:file_name", get(get_file))
        .with_state(conn)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024 * 20));
    //                               ^ sets max filesize to 20 GB

    println!("Starting server at {}", *SERVER_DOMAIN);
    // run it with hyper on localhost:3000
    let listener = TcpListener::bind("0.0.0.0:9800").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
