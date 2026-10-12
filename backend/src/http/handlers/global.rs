use aily_models::Session;
use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use log::error;
use uuid::Uuid;

use crate::{
    errors::{AilyError, HttpStatusCode, MaybeStatusCode, ProxyError},
    game::GameSession,
    get_new_session_exptime,
    http::{AppState, HttpAilyPost, HttpSession, LoggedSession, ProxyQuery},
    make_new_session_cookie,
};

pub async fn start_new_session(
    State(state): State<AppState>,
    cookies: CookieJar,
) -> Result<(CookieJar, Json<HttpSession>), AilyError> {
    let id = Uuid::new_v4().to_string();

    let post = state
        .database
        .get_random_post(false)
        .await
        .inspect_err(|e| error!("Error while getting random post in start: {}", e))?;

    let session = GameSession::new(vec![], *post.id(), get_new_session_exptime());

    state.sessions.write().await.insert(id.clone(), session);

    let cookies = cookies.add(make_new_session_cookie(id));

    Ok((
        cookies,
        Json(HttpSession::from(&Session::new(vec![], post))),
    ))
}

pub async fn current_session_post(
    State(state): State<AppState>,
    logged_session: LoggedSession,
) -> Result<(CookieJar, Json<HttpAilyPost>), AilyError> {
    let session_id = logged_session.1;

    let current = {
        let sessions = state.sessions.read().await;
        let session = sessions.get(&session_id).ok_or(AilyError::InvalidSession)?;

        *session.current()
    };

    let post = state
        .database
        .get_post_by_id(current)
        .await
        .inspect_err(|e| error!("Error getting post in current session post: {}", e))
        .map_err(|_| AilyError::Unknown)?;

    Ok((logged_session.0, Json(HttpAilyPost::from(&post))))
}

pub async fn proxy_image_handler(
    State(state): State<AppState>,
    Query(query): Query<ProxyQuery>,
) -> Result<impl IntoResponse, AilyError> {
    let client = reqwest::Client::builder()
        .user_agent(format!("Aily /1.0 (user #{})", state.danbooru_id))
        .build()
        .map_err(|_| AilyError::Unknown)?;

    let resp = client
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

    let bytes = resp
        .bytes()
        .await
        .map_err(|_| AilyError::ProxyError(ProxyError::UnableToReadResponse))?;

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_str(&content_type).unwrap(),
    );
    headers.insert(
        axum::http::header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=86400"),
    );

    Ok((headers, bytes).into_response())
}

pub async fn get_current_session(
    State(state): State<AppState>,
    logged_session: LoggedSession,
) -> Result<(CookieJar, Json<HttpSession>), AilyError> {
    let session_id = logged_session.1;

    let session = {
        let sessions = state.sessions.read().await;
        sessions
            .get(&session_id)
            .ok_or(AilyError::InvalidSession)?
            .clone()
    };

    let game_session = state
        .database
        .get_session(&session)
        .await
        .inspect_err(|e| error!("Invalid session from database: {}", e))?;

    Ok((logged_session.0, Json(HttpSession::from(&game_session))))
}
