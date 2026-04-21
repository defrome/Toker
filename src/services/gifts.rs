use crate::{
    db,
    errors::ApiError,
    models::{CreateGiftRequest, Gift, UpdateGiftRequest},
    AppState,
};

pub async fn create(state: &AppState, mut payload: CreateGiftRequest) -> Result<Gift, ApiError> {
    if payload.slug.trim().is_empty() || payload.name.trim().is_empty() {
        return Err(ApiError::bad_request("slug and name are required"));
    }
    if payload.price < 0 {
        return Err(ApiError::bad_request("price cannot be negative"));
    }
    if let Some(image_url) = payload.image_url.clone() {
        if image_url.trim().is_empty() {
            payload.image_url = None;
        }
    }

    let conn = state.db.connection()?;
    db::gifts::create(&conn, &payload).await
}

pub async fn list(state: &AppState) -> Result<Vec<Gift>, ApiError> {
    let conn = state.db.connection()?;
    db::gifts::list(&conn).await
}

pub async fn get(state: &AppState, id: i64) -> Result<Gift, ApiError> {
    let conn = state.db.connection()?;
    db::gifts::get_by_id(&conn, id).await
}

pub async fn update(
    state: &AppState,
    id: i64,
    payload: UpdateGiftRequest,
) -> Result<Gift, ApiError> {
    let conn = state.db.connection()?;
    let mut gift = db::gifts::get_by_id(&conn, id).await?;

    if let Some(slug) = payload.slug {
        if slug.trim().is_empty() {
            return Err(ApiError::bad_request("slug cannot be empty"));
        }
        gift.slug = slug;
    }
    if let Some(name) = payload.name {
        if name.trim().is_empty() {
            return Err(ApiError::bad_request("name cannot be empty"));
        }
        gift.name = name;
    }
    if let Some(description) = payload.description {
        gift.description = description;
    }
    if let Some(image_url) = payload.image_url {
        if image_url.trim().is_empty() {
            gift.image_url = None;
        } else {
            gift.image_url = Some(image_url);
        }
    }
    if let Some(price) = payload.price {
        if price < 0 {
            return Err(ApiError::bad_request("price cannot be negative"));
        }
        gift.price = price;
    }
    if let Some(rarity_level) = payload.rarity_level {
        gift.rarity_level = rarity_level;
    }
    if let Some(is_available) = payload.is_available {
        gift.is_available = is_available;
    }

    db::gifts::update(&conn, &gift).await
}

pub async fn delete(state: &AppState, id: i64) -> Result<(), ApiError> {
    let conn = state.db.connection()?;
    db::gifts::delete(&conn, id).await
}
