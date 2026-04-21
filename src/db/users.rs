use libsql::{params, Connection, Row};

use crate::{
    errors::ApiError,
    models::{UpsertUserRequest, User},
};

pub async fn upsert(conn: &Connection, payload: &UpsertUserRequest) -> Result<User, ApiError> {
    conn.execute(
        "INSERT INTO users (tg_id, username, wallet_address) VALUES (?1, ?2, ?3)
         ON CONFLICT(tg_id) DO UPDATE SET
             username = excluded.username,
             wallet_address = excluded.wallet_address",
        params![
            payload.tg_id,
            payload.username.clone(),
            payload.wallet_address.clone()
        ],
    )
    .await?;

    get_by_tg_id(conn, payload.tg_id).await
}

pub async fn get_by_tg_id(conn: &Connection, tg_id: i64) -> Result<User, ApiError> {
    let mut rows = conn
        .query(
            "SELECT tg_id, username, wallet_address, created_at FROM users WHERE tg_id = ?1",
            params![tg_id],
        )
        .await?;

    let Some(row) = rows.next().await? else {
        return Err(ApiError::not_found(format!(
            "user with tg_id={tg_id} not found"
        )));
    };

    row_to_user(&row)
}

fn row_to_user(row: &Row) -> Result<User, ApiError> {
    Ok(User {
        tg_id: row.get(0)?,
        username: row.get(1).ok(),
        wallet_address: row.get(2).ok(),
        created_at: row.get(3)?,
    })
}
