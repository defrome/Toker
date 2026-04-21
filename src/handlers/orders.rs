use axum::{
    extract::{Path, State},
    routing::{get, patch, post},
    Json, Router,
};

use crate::{
    errors::{ApiError, ApiErrorBody},
    models::{Order, PurchaseRequest, PurchaseResponse, UpdateOrderStatusRequest},
    services, AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/orders/purchase", post(purchase))
        .route("/api/orders/:id", get(get_order))
        .route("/api/orders/:id/status", patch(update_order_status))
}

#[utoipa::path(
    post,
    path = "/api/orders/purchase",
    tag = "Orders",
    request_body = PurchaseRequest,
    responses(
        (status = 200, description = "Purchase created", body = PurchaseResponse),
        (status = 400, description = "Bad request", body = ApiErrorBody),
        (status = 404, description = "Not found", body = ApiErrorBody),
        (status = 409, description = "Conflict", body = ApiErrorBody)
    )
)]
pub async fn purchase(
    State(state): State<AppState>,
    Json(payload): Json<PurchaseRequest>,
) -> Result<Json<PurchaseResponse>, ApiError> {
    let response = services::orders::purchase(&state, payload).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/orders/{id}",
    tag = "Orders",
    params(("id" = i64, Path, description = "Order id")),
    responses(
        (status = 200, description = "Order", body = Order),
        (status = 404, description = "Not found", body = ApiErrorBody)
    )
)]
pub async fn get_order(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Order>, ApiError> {
    let order = services::orders::get(&state, id).await?;
    Ok(Json(order))
}

#[utoipa::path(
    patch,
    path = "/api/orders/{id}/status",
    tag = "Orders",
    params(("id" = i64, Path, description = "Order id")),
    request_body = UpdateOrderStatusRequest,
    responses(
        (status = 200, description = "Order updated", body = Order),
        (status = 404, description = "Not found", body = ApiErrorBody)
    )
)]
pub async fn update_order_status(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateOrderStatusRequest>,
) -> Result<Json<Order>, ApiError> {
    let order = services::orders::update_status(&state, id, payload).await?;
    Ok(Json(order))
}
