use crate::models::medicine::Medicine;
use sqlx::{Result, SqlitePool};

pub async fn get_medicines(pool: &SqlitePool) -> Result<Vec<Medicine>> {
    let rows = sqlx::query_as::<_, Medicine>("SELECT id, name, quantity FROM medicines")
        .fetch_all(pool)
        .await?;

    Ok(rows)
}
