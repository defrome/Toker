use libsql::{params, Connection};

use crate::errors::ApiError;

pub async fn store(
    conn: &Connection,
    user_id: i64,
    token_hash: &str,
    expires_at: i64,
) -> Result<(), ApiError> {
    conn.execute(
        "INSERT INTO auth_refresh_tokens (user_id, token_hash, expires_at) VALUES (?1, ?2, ?3)",
        params![user_id, token_hash, expires_at],
    )
    .await?;
    Ok(())
}

pub async fn consume(
    conn: &Connection,
    user_id: i64,
    token_hash: &str,
    now_ts: i64,
) -> Result<(), ApiError> {
    let affected = conn
        .execute(
            "UPDATE auth_refresh_tokens
             SET revoked_at = CURRENT_TIMESTAMP
             WHERE user_id = ?1
               AND token_hash = ?2
               AND revoked_at IS NULL
               AND expires_at > ?3",
            params![user_id, token_hash, now_ts],
        )
        .await?;

    if affected == 0 {
        return Err(ApiError::unauthorized(
            "invalid or already used refresh token",
        ));
    }

    Ok(())
}
