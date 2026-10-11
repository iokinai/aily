use core::fmt;
use std::error::Error;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};

#[derive(Debug)]
pub enum AilyError {
    NotFound,
    InvalidSession,
    ProxyError,
    Unknown,
}

impl Error for AilyError {}

impl fmt::Display for AilyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AilyError::NotFound => write!(f, "Not found"),
            AilyError::Unknown => write!(f, "Unknown"),
            AilyError::InvalidSession => write!(f, "Invalid session"),
            AilyError::ProxyError => write!(f, "Proxy error"),
        }
    }
}

impl IntoResponse for AilyError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AilyError::NotFound => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not found"))
                .unwrap(),
            AilyError::Unknown => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Unknown"))
                .unwrap(),
            AilyError::InvalidSession => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Invalid session"))
                .unwrap(),
            AilyError::ProxyError => Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::from("Proxy error"))
                .unwrap(),
        }
    }
}
