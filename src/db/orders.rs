use libsql::{params, Connection, Row, TransactionBehavior};

use crate::{
    errors::ApiError,
    models::{Order, OrderStatus},
};

pub async fn create_pending_order_and_mark_sold(
    conn: &Connection,
    user_id: i64,
    gift_id: i64,
) -> Result<Order, ApiError> {
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .await?;

    let mut user_rows = tx
        .query("SELECT tg_id FROM users WHERE tg_id = ?1", params![user_id])
        .await?;

    if user_rows.next().await?.is_none() {
        return Err(ApiError::not_found(format!(
            "user with tg_id={user_id} not found"
        )));
    }

    let mut gift_rows = tx
        .query(
            "SELECT is_available FROM gifts WHERE id = ?1",
            params![gift_id],
        )
        .await?;

    let Some(gift_row) = gift_rows.next().await? else {
        return Err(ApiError::not_found(format!(
            "gift with id={gift_id} not found"
        )));
    };

    let is_available = gift_row.get::<i64>(0)? == 1;
    if !is_available {
        return Err(ApiError::conflict("gift is already sold"));
    }

    tx.execute(
        "INSERT INTO orders (user_id, gift_id, status, tx_hash) VALUES (?1, ?2, 'pending', NULL)",
        params![user_id, gift_id],
    )
    .await?;

    let affected = tx
        .execute(
            "UPDATE gifts SET is_available = 0 WHERE id = ?1 AND is_available = 1",
            params![gift_id],
        )
        .await?;

    if affected == 0 {
        return Err(ApiError::conflict("gift was sold concurrently"));
    }

    let order_id = tx.last_insert_rowid();
    tx.commit().await?;

    get_by_id(conn, order_id).await
}

pub async fn get_by_id(conn: &Connection, id: i64) -> Result<Order, ApiError> {
    let mut rows = conn
        .query(
            "SELECT id, user_id, gift_id, status, tx_hash, created_at FROM orders WHERE id = ?1",
            params![id],
        )
        .await?;

    let Some(row) = rows.next().await? else {
        return Err(ApiError::not_found(format!("order with id={id} not found")));
    };

    row_to_order(&row)
}

pub async fn get_by_id_for_user(
    conn: &Connection,
    id: i64,
    user_id: i64,
) -> Result<Order, ApiError> {
    let mut rows = conn
        .query(
            "SELECT id, user_id, gift_id, status, tx_hash, created_at FROM orders WHERE id = ?1 AND user_id = ?2",
            params![id, user_id],
        )
        .await?;

    let Some(row) = rows.next().await? else {
        return Err(ApiError::not_found(format!(
            "order with id={id} for user_id={user_id} not found"
        )));
    };

    row_to_order(&row)
}

pub async fn update_status_for_user(
    conn: &Connection,
    id: i64,
    user_id: i64,
    status: OrderStatus,
    tx_hash: Option<String>,
) -> Result<Order, ApiError> {
    let affected = conn
        .execute(
            "UPDATE orders SET status = ?1, tx_hash = ?2 WHERE id = ?3 AND user_id = ?4",
            params![status.as_str(), tx_hash, id, user_id],
        )
        .await?;

    if affected == 0 {
        return Err(ApiError::not_found(format!(
            "order with id={id} for user_id={user_id} not found"
        )));
    }

    get_by_id(conn, id).await
}

fn row_to_order(row: &Row) -> Result<Order, ApiError> {
    let status_text: String = row.get(3)?;
    let status = OrderStatus::parse(&status_text)
        .ok_or_else(|| ApiError::internal(format!("invalid order status in db: {status_text}")))?;

    Ok(Order {
        id: row.get(0)?,
        user_id: row.get(1)?,
        gift_id: row.get(2)?,
        status,
        tx_hash: row.get(4).ok(),
        created_at: row.get(5)?,
    })
}
