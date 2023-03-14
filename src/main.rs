mod constants;
mod db;
mod helpers;
mod models;
mod routes;
mod views;

use constants::ROOT_FOLDER;
use helpers::scheduled_deletion;
use routes::{delete_file_route, download_file, get_all_files, get_file, upload_file};
use tower_http::services::ServeDir;
use views::{all_files, root, upload};

use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
    Router,
};
use sqlx::SqlitePool;
use std::{fs::create_dir, path, time::Duration};

use crate::constants::SERVER_DOMAIN;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Couldn't find .env file");

    // Create files and folders
    if !path::Path::new(ROOT_FOLDER).exists() {
        create_dir(path::Path::new(ROOT_FOLDER)).expect("couldn't create folder 'files'");
    }
    if !path::Path::new("db").exists() {
        create_dir(path::Path::new("db")).expect("couldn't create folder 'db'");
    }
    if !path::Path::new("db/files.db").exists() {
        std::fs::File::create("db/files.db").unwrap();
    }

    // Create database pool
    let conn = SqlitePool::connect("sqlite://db/files.db")
        .await
        .expect("No DB Pool");

    tokio::spawn(async {
        let conn2 = SqlitePool::connect("sqlite://db/files.db")
            .await
            .expect("No DB Pool");

        loop {
            scheduled_deletion(conn2.clone()).await;
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });

    // Set up routes
    let app = Router::new()
        .route("/", get(root))
        .route("/", post(download_file))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/upload", get(upload))
        .route("/upload", post(upload_file))
        .route("/:file_name", get(get_file))
        .route("/delete", post(delete_file_route))
        .route("/files", get(all_files))
        .with_state(conn)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024 * 20));
    //                               ^ sets max filesize to 20 GB

    println!("Starting server at {}", *SERVER_DOMAIN);
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
