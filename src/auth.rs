use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{db, errors::ApiError, AppState};

const ACCESS_TOKEN_TTL_SECS: u64 = 60 * 60;
const REFRESH_TOKEN_TTL_SECS: u64 = 30 * 24 * 60 * 60;
const ACCESS_TOKEN_TYPE: &str = "access";
const REFRESH_TOKEN_TYPE: &str = "refresh";

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub access_secret: String,
    pub refresh_secret: String,
    pub issuer: String,
    pub audience: String,
}

impl FromRef<AppState> for AuthConfig {
    fn from_ref(state: &AppState) -> Self {
        state.auth.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
    pub iat: usize,
    pub nbf: usize,
    pub iss: String,
    pub aud: String,
    pub jti: String,
    pub token_type: String,
}

#[derive(Debug, Clone, Copy)]
pub struct AuthUser {
    pub tg_id: i64,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AuthConfig: FromRef<S>,
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

        let config = AuthConfig::from_ref(state);
        let tg_id = parse_access_token(&config, token)?;

        Ok(Self { tg_id })
    }
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AuthTokensResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

pub async fn issue_token_pair(
    state: &AppState,
    tg_id: i64,
) -> Result<AuthTokensResponse, ApiError> {
    let access_token = issue_token(&state.auth, tg_id, ACCESS_TOKEN_TTL_SECS, ACCESS_TOKEN_TYPE)?;
    let refresh_token = issue_token(
        &state.auth,
        tg_id,
        REFRESH_TOKEN_TTL_SECS,
        REFRESH_TOKEN_TYPE,
    )?;

    let claims = parse_refresh_claims(&state.auth, &refresh_token)?;
    let conn = state.db.connection()?;
    db::auth_refresh_tokens::store(&conn, tg_id, &hash_token(&refresh_token), claims.exp as i64)
        .await?;

    Ok(AuthTokensResponse {
        access_token,
        refresh_token,
        expires_in: ACCESS_TOKEN_TTL_SECS,
        token_type: "Bearer".to_string(),
    })
}

pub fn parse_refresh_token(config: &AuthConfig, token: &str) -> Result<i64, ApiError> {
    let claims = parse_refresh_claims(config, token)?;
    if claims.sub <= 0 {
        return Err(ApiError::unauthorized("invalid token subject"));
    }
    Ok(claims.sub)
}

pub async fn consume_refresh_token(
    state: &AppState,
    token: &str,
    tg_id: i64,
) -> Result<(), ApiError> {
    let now_ts = now_ts()?;
    let conn = state.db.connection()?;
    db::auth_refresh_tokens::consume(&conn, tg_id, &hash_token(token), now_ts).await
}

fn parse_refresh_claims(config: &AuthConfig, token: &str) -> Result<Claims, ApiError> {
    let claims = decode_claims(
        &config.refresh_secret,
        token,
        REFRESH_TOKEN_TYPE,
        &config.issuer,
        &config.audience,
    )?;

    if claims.sub <= 0 {
        return Err(ApiError::unauthorized("invalid token subject"));
    }

    Ok(claims)
}

fn parse_access_token(config: &AuthConfig, token: &str) -> Result<i64, ApiError> {
    let claims = decode_claims(
        &config.access_secret,
        token,
        ACCESS_TOKEN_TYPE,
        &config.issuer,
        &config.audience,
    )?;

    if claims.sub <= 0 {
        return Err(ApiError::unauthorized("invalid token subject"));
    }

    Ok(claims.sub)
}

fn issue_token(
    config: &AuthConfig,
    tg_id: i64,
    ttl_secs: u64,
    token_type: &str,
) -> Result<String, ApiError> {
    let now = now_ts()?;
    let exp = now
        .checked_add(i64::try_from(ttl_secs).map_err(|_| ApiError::internal("ttl overflow"))?)
        .ok_or_else(|| ApiError::internal("token expiration overflow"))?;

    let secret = if token_type == ACCESS_TOKEN_TYPE {
        &config.access_secret
    } else {
        &config.refresh_secret
    };

    let claims = Claims {
        sub: tg_id,
        exp: exp as usize,
        iat: now as usize,
        nbf: now as usize,
        iss: config.issuer.clone(),
        aud: config.audience.clone(),
        jti: Uuid::new_v4().to_string(),
        token_type: token_type.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ApiError::internal(format!("failed to issue jwt: {e}")))
}

fn decode_claims(
    secret: &str,
    token: &str,
    expected_type: &str,
    issuer: &str,
    audience: &str,
) -> Result<Claims, ApiError> {
    let mut validation = Validation::default();
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[audience]);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(|_| ApiError::unauthorized("invalid or expired token"))?;

    if token_data.claims.token_type != expected_type {
        return Err(ApiError::unauthorized("invalid token type"));
    }

    Ok(token_data.claims)
}

fn now_ts() -> Result<i64, ApiError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| ApiError::internal(format!("system clock error: {e}")))?;

    i64::try_from(now.as_secs()).map_err(|_| ApiError::internal("timestamp overflow"))
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}
