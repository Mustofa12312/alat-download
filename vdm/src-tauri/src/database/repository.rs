use sqlx::SqlitePool;
use super::models::DownloadRecord;

pub struct DownloadRepository;

impl DownloadRepository {
    pub async fn create_download(
        pool: &SqlitePool,
        url: &str,
        file_name: &str,
        file_path: &str,
        total_size: i64,
    ) -> Result<i32, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO downloads (url, file_name, file_path, total_size, status)
            VALUES (?, ?, ?, ?, 0)
            "#,
        )
        .bind(url)
        .bind(file_name)
        .bind(file_path)
        .bind(total_size)
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid() as i32)
    }

    pub async fn update_progress(
        pool: &SqlitePool,
        id: i32,
        downloaded_size: i64,
        speed: f64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE downloads 
            SET downloaded_size = ?, download_speed = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(downloaded_size)
        .bind(speed)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_status(
        pool: &SqlitePool,
        id: i32,
        status: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE downloads 
            SET status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<DownloadRecord>, sqlx::Error> {
        let rows = sqlx::query_as::<_, DownloadRecord>("SELECT * FROM downloads ORDER BY created_at DESC")
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }

    pub async fn delete_download(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM downloads WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
            
        Ok(())
    }
}
