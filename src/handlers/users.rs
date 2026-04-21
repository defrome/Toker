use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::{
    errors::{ApiError, ApiErrorBody},
    models::{UpsertUserRequest, User},
    services, AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/users", post(upsert_user))
        .route("/api/users/:tg_id", get(get_user))
}

#[utoipa::path(
    post,
    path = "/api/users",
    tag = "Users",
    request_body = UpsertUserRequest,
    responses(
        (status = 201, description = "User created/updated", body = User),
        (status = 400, description = "Bad request", body = ApiErrorBody)
    )
)]
pub async fn upsert_user(
    State(state): State<AppState>,
    Json(payload): Json<UpsertUserRequest>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    let user = services::users::upsert(&state, payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    get,
    path = "/api/users/{tg_id}",
    tag = "Users",
    params(("tg_id" = i64, Path, description = "Telegram user id")),
    responses(
        (status = 200, description = "User", body = User),
        (status = 404, description = "Not found", body = ApiErrorBody)
    )
)]
pub async fn get_user(
    State(state): State<AppState>,
    Path(tg_id): Path<i64>,
) -> Result<Json<User>, ApiError> {
    let user = services::users::get(&state, tg_id).await?;
    Ok(Json(user))
}
