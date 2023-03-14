use sqlx::{types::chrono, Pool, Sqlite};

use crate::db::delete_file;

pub async fn scheduled_deletion(db: Pool<Sqlite>) {
    let timestamp = chrono::Utc::now();

    let files = match sqlx::query!(
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
        Err(_) => return,
    };

    for file in files.iter() {
        println!(
            "Deleted {} with timestamp {:#?}",
            file.file_name.clone(),
            file.destroy.clone()
        );
        delete_file(db.clone(), file.file_name.clone()).await;
    }
}
