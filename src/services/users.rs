use crate::{
    db,
    errors::ApiError,
    models::{UpsertUserRequest, User},
    AppState,
};

pub async fn upsert(state: &AppState, payload: UpsertUserRequest) -> Result<User, ApiError> {
    if payload.tg_id <= 0 {
        return Err(ApiError::bad_request("tg_id must be positive i64"));
    }

    let conn = state.db.connection()?;
    db::users::upsert(&conn, &payload).await
}

pub async fn get(state: &AppState, tg_id: i64) -> Result<User, ApiError> {
    let conn = state.db.connection()?;
    db::users::get_by_tg_id(&conn, tg_id).await
}
