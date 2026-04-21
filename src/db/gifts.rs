use libsql::{params, Connection, Row};

use crate::{
    errors::ApiError,
    models::{CreateGiftRequest, Gift},
};

pub async fn create(conn: &Connection, payload: &CreateGiftRequest) -> Result<Gift, ApiError> {
    conn.execute(
        "INSERT INTO gifts (slug, name, description, image_url, price, rarity_level, is_available) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            payload.slug.as_str(),
            payload.name.as_str(),
            payload.description.as_str(),
            payload.image_url.clone(),
            payload.price,
            payload.rarity_level.as_str(),
            if payload.is_available { 1 } else { 0 }
        ],
    )
    .await?;

    let id = conn.last_insert_rowid();
    get_by_id(conn, id).await
}

pub async fn list(conn: &Connection) -> Result<Vec<Gift>, ApiError> {
    let mut rows = conn
        .query(
            "SELECT id, slug, name, description, image_url, price, rarity_level, is_available FROM gifts ORDER BY id DESC",
            (),
        )
        .await?;

    let mut gifts = Vec::new();
    while let Some(row) = rows.next().await? {
        gifts.push(row_to_gift(&row)?);
    }

    Ok(gifts)
}

pub async fn get_by_id(conn: &Connection, id: i64) -> Result<Gift, ApiError> {
    let mut rows = conn
        .query(
            "SELECT id, slug, name, description, image_url, price, rarity_level, is_available FROM gifts WHERE id = ?1",
            params![id],
        )
        .await?;

    let Some(row) = rows.next().await? else {
        return Err(ApiError::not_found(format!("gift with id={id} not found")));
    };

    row_to_gift(&row)
}

pub async fn update(conn: &Connection, gift: &Gift) -> Result<Gift, ApiError> {
    let affected = conn
        .execute(
            "UPDATE gifts SET slug = ?1, name = ?2, description = ?3, image_url = ?4, price = ?5, rarity_level = ?6, is_available = ?7 WHERE id = ?8",
            params![
                gift.slug.as_str(),
                gift.name.as_str(),
                gift.description.as_str(),
                gift.image_url.clone(),
                gift.price,
                gift.rarity_level.as_str(),
                if gift.is_available { 1 } else { 0 },
                gift.id,
            ],
        )
        .await?;

    if affected == 0 {
        return Err(ApiError::not_found(format!(
            "gift with id={} not found",
            gift.id
        )));
    }

    get_by_id(conn, gift.id).await
}

pub async fn delete(conn: &Connection, id: i64) -> Result<(), ApiError> {
    let affected = conn
        .execute("DELETE FROM gifts WHERE id = ?1", params![id])
        .await?;

    if affected == 0 {
        return Err(ApiError::not_found(format!("gift with id={id} not found")));
    }

    Ok(())
}

fn row_to_gift(row: &Row) -> Result<Gift, ApiError> {
    Ok(Gift {
        id: row.get(0)?,
        slug: row.get(1)?,
        name: row.get(2)?,
        description: row.get(3)?,
        image_url: row.get(4).ok(),
        price: row.get(5)?,
        rarity_level: row.get(6)?,
        is_available: row.get::<i64>(7)? == 1,
    })
}
