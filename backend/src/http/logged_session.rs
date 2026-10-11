use axum::{extract::FromRequestParts, http::request::Parts};
use reqwest::StatusCode;

use crate::{get_new_session_lifetime, http::AppState};

pub struct LoggedSession(pub String);

impl FromRequestParts<AppState> for LoggedSession {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let session_id = headers
            .get("X-Session-ID")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let mut sessions = state.sessions.write().await;

        if let Some(session) = sessions.get_mut(session_id) {
            session.set_expires_at(get_new_session_lifetime());
            return Ok(LoggedSession(session_id.to_string()));
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
}
