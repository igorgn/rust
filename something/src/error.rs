use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

/// Custom error type for the URL shortener
/// 
/// This demonstrates:
/// - thiserror crate for ergonomic error handling
/// - Converting domain errors to HTTP responses
/// - Separating error concerns from business logic
#[derive(Debug, Error)]
pub enum AppError {
    #[error("URL not found: {0}")]
    NotFound(String),
    
    #[error("Invalid request: {0}")]
    BadRequest(String),
    
    #[error("URL already exists")]
    Conflict,
    
    #[error("Internal storage error: {0}")]
    StorageError(String),
    
    #[error("Internal server error")]
    InternalError,
}

/// Convert AppError into an HTTP response
/// 
/// This trait implementation allows Axum handlers to return Result<T, AppError>
/// and have errors automatically converted to proper HTTP responses.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Conflict => (StatusCode::CONFLICT, "Resource already exists".to_string()),
            AppError::StorageError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

/// Convert anyhow::Error to AppError
/// 
/// This allows using ? operator with anyhow errors in handlers
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError
    }
}
