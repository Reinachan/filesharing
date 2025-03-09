use axum::http::StatusCode;
use sqlx::{Pool, Sqlite, types::chrono};

use crate::{
    db::{delete_file, delete_user_db},
    models::{FileDB, UserDB},
};

pub async fn scheduled_deletion(db: Pool<Sqlite>) -> Result<(), (StatusCode, String)> {
    let timestamp = chrono::Utc::now();

    let files = match sqlx::query_as!(
        FileDB,
        "
    SELECT * FROM files
    WHERE destroy < ? 
    ",
        timestamp
    )
    .fetch_all(&db)
    .await
    {
        Ok(files) => files,
        Err(err) => {
            println!("{err:#?}");
            return Ok(());
        }
    };

    for file in files.iter() {
        println!(
            "Deleted file: {}, with timestamp: {:#?}",
            file.file_name.clone(),
            file.destroy.unwrap(),
        );
        delete_file(&db, file.saved_name.clone()).await?;
    }

    match sqlx::query_as!(
        UserDB,
        "
    SELECT * FROM users
    WHERE terminate < ? 
    ",
        timestamp
    )
    .fetch_all(&db)
    .await
    {
        Ok(users) => {
            for user in users.iter() {
                println!(
                    "Deleted user: {}, with timestamp: {:#?}",
                    user.username,
                    user.terminate.unwrap()
                );
                delete_user_db(&db, user.username.clone()).await?;
            }
        }
        Err(_) => todo!(),
    };

    Ok(())
}
