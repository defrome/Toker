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

    // Backward-compatible migration for existing databases created before `currency`.
    if let Err(err) = conn
        .execute(
            "ALTER TABLE gifts ADD COLUMN currency TEXT NOT NULL DEFAULT 'stars' CHECK (currency IN ('stars', 'rub'))",
            (),
        )
        .await
    {
        let msg = err.to_string();
        if !msg.contains("duplicate column name") {
            return Err(ApiError::from(err));
        }
    }

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS auth_refresh_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token_hash TEXT NOT NULL UNIQUE,
            expires_at INTEGER NOT NULL,
            revoked_at TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(tg_id)
        );
        CREATE INDEX IF NOT EXISTS idx_auth_refresh_tokens_user_id ON auth_refresh_tokens(user_id);",
    )
    .await?;

    Ok(())
}
