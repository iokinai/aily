use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use reqwest::StatusCode;

use crate::{get_new_session_exptime, http::AppState, make_new_session_cookie};

pub struct LoggedSession(pub CookieJar, pub String);

impl FromRequestParts<AppState> for LoggedSession {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let cookies = CookieJar::from_headers(headers);
        let session_id_cookie = &cookies.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;
        let session_id = session_id_cookie.value();

        let mut sessions = state.sessions.write().await;

        if let Some(session) = sessions.get_mut(session_id) {
            let cookies = cookies.clone();
            let cookies = cookies.add(make_new_session_cookie(session_id.to_string()));
            session.set_expires_at(get_new_session_exptime());
            return Ok(LoggedSession(cookies, session_id.to_string()));
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
}
