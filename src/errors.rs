use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorBody {
    pub error: String,
}

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            message: message.into(),
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: message.into(),
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status;
        let body = Json(ApiErrorBody {
            error: self.message,
        });

        (status, body).into_response()
    }
}

impl From<libsql::Error> for ApiError {
    fn from(value: libsql::Error) -> Self {
        Self::internal(format!("database error: {value}"))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        Self::internal(format!("io error: {value}"))
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        Self::internal(format!("internal error: {value}"))
    }
}
