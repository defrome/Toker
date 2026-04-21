use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::{
    auth::AuthUser,
    errors::{ApiError, ApiErrorBody},
    models::{CreateGiftRequest, Gift, UpdateGiftRequest},
    services, AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/gifts", post(create_gift).get(list_gifts))
        .route(
            "/api/gifts/:id",
            get(get_gift).put(update_gift).delete(delete_gift),
        )
}

#[utoipa::path(
    post,
    path = "/api/gifts",
    tag = "Gifts",
    security(("bearer_auth" = [])),
    request_body = CreateGiftRequest,
    responses(
        (status = 201, description = "Gift created", body = Gift),
        (status = 400, description = "Bad request", body = ApiErrorBody),
        (status = 500, description = "Internal error", body = ApiErrorBody)
    )
)]
pub async fn create_gift(
    _auth: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateGiftRequest>,
) -> Result<(StatusCode, Json<Gift>), ApiError> {
    let gift = services::gifts::create(&state, payload).await?;
    Ok((StatusCode::CREATED, Json(gift)))
}

#[utoipa::path(
    get,
    path = "/api/gifts",
    tag = "Gifts",
    security(("bearer_auth" = [])),
    responses((status = 200, description = "Gift list", body = [Gift]))
)]
pub async fn list_gifts(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<Gift>>, ApiError> {
    let gifts = services::gifts::list(&state).await?;
    Ok(Json(gifts))
}

#[utoipa::path(
    get,
    path = "/api/gifts/{id}",
    tag = "Gifts",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Gift id")),
    responses(
        (status = 200, description = "Gift", body = Gift),
        (status = 404, description = "Not found", body = ApiErrorBody)
    )
)]
pub async fn get_gift(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Gift>, ApiError> {
    let gift = services::gifts::get(&state, id).await?;
    Ok(Json(gift))
}

#[utoipa::path(
    put,
    path = "/api/gifts/{id}",
    tag = "Gifts",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Gift id")),
    request_body = UpdateGiftRequest,
    responses(
        (status = 200, description = "Gift updated", body = Gift),
        (status = 400, description = "Bad request", body = ApiErrorBody),
        (status = 404, description = "Not found", body = ApiErrorBody)
    )
)]
pub async fn update_gift(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateGiftRequest>,
) -> Result<Json<Gift>, ApiError> {
    let gift = services::gifts::update(&state, id, payload).await?;
    Ok(Json(gift))
}

#[utoipa::path(
    delete,
    path = "/api/gifts/{id}",
    tag = "Gifts",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Gift id")),
    responses(
        (status = 204, description = "Gift deleted"),
        (status = 404, description = "Not found", body = ApiErrorBody)
    )
)]
pub async fn delete_gift(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    services::gifts::delete(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
