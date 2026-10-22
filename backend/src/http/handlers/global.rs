use aily_models::Session;
use axum::{
    Json,
    body::Body,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use log::error;
use uuid::Uuid;

use crate::{
    errors::{AilyError, HttpStatusCode, MaybeStatusCode, ProxyError},
    http::{AppState, HttpSession, LoggedSession, ProxyRequest},
    make_new_session_cookie,
};

pub async fn start_new_session(
    State(state): State<AppState>,
    cookies: CookieJar,
) -> Result<(CookieJar, Json<HttpSession>), AilyError> {
    let id = Uuid::new_v4().to_string();

    state
        .global_game_controller
        .make_and_save_new_session(&id)
        .await;

    let cookies = cookies.add(make_new_session_cookie(id));

    Ok((
        cookies,
        Json(HttpSession::from(&Session::new(vec![], vec![], None, None))),
    ))
}

pub async fn get_current_session(
    State(state): State<AppState>,
    logged_session: LoggedSession,
) -> Result<(CookieJar, Json<HttpSession>), AilyError> {
    let session_id = logged_session.1;

    let session = state
        .global_game_controller
        .get_session_by_id(&session_id)
        .await?;

    let game_session = state
        .global_game_controller
        .game_session_to_global_session(&session)
        .await
        .inspect_err(|e| error!("Invalid session from database: {}", e))?;

    Ok((logged_session.0, Json(HttpSession::from(&game_session))))
}

pub async fn proxy_image_handler(
    State(state): State<AppState>,
    Query(query): Query<ProxyRequest>,
) -> Result<impl IntoResponse, AilyError> {
    let resp = state
        .http_request_client
        .get(query.url())
        .send()
        .await
        .inspect_err(|e| {
            let s = if let Some(s) = e.status() {
                s.to_string()
            } else {
                "<no status>".to_string()
            };
            error!("Error while sending request in proxy: {}", s)
        })
        .map_err(|e| {
            AilyError::ProxyError(ProxyError::CouldNotSendRequest {
                status: MaybeStatusCode::from_option(e.status()),
            })
        })?;

    if !resp.status().is_success() {
        error!("Unknown error in proxy with status: {}", resp.status());

        return Err(AilyError::ProxyError(ProxyError::RequestFailed {
            status: HttpStatusCode(resp.status().as_u16()),
        }));
    }

    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_str(&content_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );

    headers.insert(
        axum::http::header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=86400"),
    );

    Ok((headers, Body::from_stream(resp.bytes_stream())).into_response())
}
