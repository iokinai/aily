use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use crate::errors::{ErrorType, NotFound, ProxyError, http_error_body::HttpErrorBody};

#[derive(Error, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AilyError {
    #[error("Not found: {0}")]
    NotFound(#[from] NotFound),
    #[error("This session is invalid")]
    InvalidSession,
    #[error("Error on proxy: {0}")]
    ProxyError(#[from] ProxyError),
    #[error("Unknown error occured")]
    Unknown,
}

impl IntoResponse for AilyError {
    fn into_response(self) -> Response {
        let (e_type, status, inner) = match self {
            AilyError::NotFound(not_found) => (
                ErrorType::NotFound,
                axum::http::StatusCode::NOT_FOUND,
                serde_json::to_value(&not_found).unwrap_or(serde_json::Value::Null),
            ),
            AilyError::InvalidSession => (
                ErrorType::InvalidSession,
                axum::http::StatusCode::BAD_REQUEST,
                serde_json::Value::Null,
            ),
            AilyError::ProxyError(proxy_error) => (
                ErrorType::ProxyError,
                proxy_error.get_status_code(),
                serde_json::to_value(&proxy_error).unwrap_or(serde_json::Value::Null),
            ),
            AilyError::Unknown => (
                ErrorType::Unknown,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::Value::Null,
            ),
        };

        (status, Json(HttpErrorBody::new(e_type, inner))).into_response()
    }
}
