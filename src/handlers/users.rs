use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::{
    auth::{issue_token, AuthTokenResponse, AuthUser},
    errors::{ApiError, ApiErrorBody},
    models::{UpsertUserRequest, User},
    services, AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/users", post(upsert_user))
        .route("/api/users/:tg_id", get(get_user))
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct UpsertUserResponse {
    pub user: User,
    pub auth: AuthTokenResponse,
}

#[utoipa::path(
    post,
    path = "/api/users",
    tag = "Users",
    security(("bearer_auth" = [])),
    request_body = UpsertUserRequest,
    responses(
        (status = 201, description = "User created/updated", body = UpsertUserResponse),
        (status = 400, description = "Bad request", body = ApiErrorBody)
    )
)]
pub async fn upsert_user(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<UpsertUserRequest>,
) -> Result<(StatusCode, Json<UpsertUserResponse>), ApiError> {
    if auth.tg_id != payload.tg_id {
        return Err(ApiError::forbidden(
            "token user does not match payload tg_id",
        ));
    }

    let user = services::users::upsert(&state, payload).await?;
    let access_token = issue_token(&state.jwt_secret, user.tg_id)?;

    Ok((
        StatusCode::CREATED,
        Json(UpsertUserResponse {
            user,
            auth: AuthTokenResponse { access_token },
        }),
    ))
}

#[utoipa::path(
    get,
    path = "/api/users/{tg_id}",
    tag = "Users",
    security(("bearer_auth" = [])),
    params(("tg_id" = i64, Path, description = "Telegram user id")),
    responses(
        (status = 200, description = "User", body = User),
        (status = 404, description = "Not found", body = ApiErrorBody)
    )
)]
pub async fn get_user(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(tg_id): Path<i64>,
) -> Result<Json<User>, ApiError> {
    if auth.tg_id != tg_id {
        return Err(ApiError::forbidden(
            "you can only access your own user profile",
        ));
    }

    let user = services::users::get(&state, tg_id).await?;
    Ok(Json(user))
}
