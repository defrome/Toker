use axum::{extract::State, routing::post, Json, Router};

use crate::{
    auth::{consume_refresh_token, issue_token_pair, parse_refresh_token, AuthTokensResponse},
    errors::{ApiError, ApiErrorBody},
    models::{UpsertUserRequest, User},
    services, AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/refresh", post(refresh))
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct LoginResponse {
    pub user: User,
    pub auth: AuthTokensResponse,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct RefreshTokenResponse {
    pub auth: AuthTokensResponse,
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Auth",
    request_body = UpsertUserRequest,
    responses(
        (status = 200, description = "Authenticated user", body = LoginResponse),
        (status = 400, description = "Bad request", body = ApiErrorBody)
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<UpsertUserRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let user = services::users::upsert(&state, payload).await?;
    let auth = issue_token_pair(&state, user.tg_id).await?;

    Ok(Json(LoginResponse { user, auth }))
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "Auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "New access and refresh tokens", body = RefreshTokenResponse),
        (status = 401, description = "Invalid refresh token", body = ApiErrorBody)
    )
)]
pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, ApiError> {
    if payload.refresh_token.trim().is_empty() {
        return Err(ApiError::bad_request("refresh_token is required"));
    }

    let tg_id = parse_refresh_token(&state.auth, &payload.refresh_token)?;
    consume_refresh_token(&state, &payload.refresh_token, tg_id).await?;
    services::users::get(&state, tg_id).await?;

    let auth = issue_token_pair(&state, tg_id).await?;
    Ok(Json(RefreshTokenResponse { auth }))
}
