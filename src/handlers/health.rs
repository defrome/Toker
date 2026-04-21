use axum::{routing::get, Json, Router};

use crate::{models::HealthResponse, AppState};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/health", get(health))
}

#[utoipa::path(
    get,
    path = "/api/health",
    tag = "Health",
    responses((status = 200, description = "Service health", body = HealthResponse))
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "telegram-nft-gift-marketplace",
    })
}
