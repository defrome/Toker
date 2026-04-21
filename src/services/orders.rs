use crate::{
    db,
    errors::ApiError,
    models::{Gift, Order, PurchaseRequest, PurchaseResponse, UpdateOrderStatusRequest},
    AppState,
};

pub async fn purchase(
    state: &AppState,
    tg_id: i64,
    payload: PurchaseRequest,
) -> Result<PurchaseResponse, ApiError> {
    if tg_id <= 0 {
        return Err(ApiError::bad_request("token subject must be positive i64"));
    }
    if payload.gift_id <= 0 {
        return Err(ApiError::bad_request("gift_id must be positive"));
    }

    let conn = state.db.connection()?;
    let order =
        db::orders::create_pending_order_and_mark_sold(&conn, tg_id, payload.gift_id).await?;
    let gift: Gift = db::gifts::get_by_id(&conn, payload.gift_id).await?;

    Ok(PurchaseResponse { order, gift })
}

pub async fn get(state: &AppState, id: i64, tg_id: i64) -> Result<Order, ApiError> {
    let conn = state.db.connection()?;
    db::orders::get_by_id_for_user(&conn, id, tg_id).await
}

pub async fn update_status(
    state: &AppState,
    id: i64,
    tg_id: i64,
    payload: UpdateOrderStatusRequest,
) -> Result<Order, ApiError> {
    if id <= 0 {
        return Err(ApiError::bad_request("order id must be positive"));
    }

    let conn = state.db.connection()?;
    db::orders::update_status_for_user(&conn, id, tg_id, payload.status, payload.tx_hash).await
}
