use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{errors::ApiError, AppState};

const TOKEN_TTL_HOURS: u64 = 24;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
}

#[derive(Clone)]
pub struct JwtSecret(pub String);

impl FromRef<AppState> for JwtSecret {
    fn from_ref(state: &AppState) -> Self {
        Self(state.jwt_secret.clone())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AuthUser {
    pub tg_id: i64,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    JwtSecret: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| ApiError::unauthorized("missing Authorization header"))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| ApiError::unauthorized("expected Bearer token"))?;

        let JwtSecret(secret) = JwtSecret::from_ref(state);

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| ApiError::unauthorized("invalid or expired token"))?;

        if token_data.claims.sub <= 0 {
            return Err(ApiError::unauthorized("invalid token subject"));
        }

        Ok(Self {
            tg_id: token_data.claims.sub,
        })
    }
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AuthTokenResponse {
    pub access_token: String,
}

pub fn issue_token(secret: &str, tg_id: i64) -> Result<String, ApiError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| ApiError::internal(format!("system clock error: {e}")))?;
    let exp = now
        .checked_add(Duration::from_secs(TOKEN_TTL_HOURS * 60 * 60))
        .ok_or_else(|| ApiError::internal("token expiration overflow"))?
        .as_secs() as usize;

    let claims = Claims { sub: tg_id, exp };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ApiError::internal(format!("failed to issue jwt: {e}")))
}
