use tokio::fs;

use crate::{db::client::Db, errors::ApiError};

pub async fn run_schema(db: &Db) -> Result<(), ApiError> {
    let sql = fs::read_to_string("schema.sql").await?;
    let conn = db.connection()?;
    conn.execute_batch(&sql).await?;

    // Backward-compatible migration for existing databases created before `image_url`.
    if let Err(err) = conn
        .execute("ALTER TABLE gifts ADD COLUMN image_url TEXT", ())
        .await
    {
        let msg = err.to_string();
        if !msg.contains("duplicate column name") {
            return Err(ApiError::from(err));
        }
    }

    Ok(())
}
