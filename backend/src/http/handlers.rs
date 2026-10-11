use aily_models::GameSession;
use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};

use axum_extra::extract::{CookieJar, cookie::Cookie};

use log::{error, warn};
use uuid::Uuid;

use crate::{
    errors::AilyError,
    game::Session,
    get_new_session_exptime, get_new_session_lifetime,
    http::{
        AppState, GuessQuery, HttpDanbooruPost, HttpSession, LoggedSession, RandomPostRequest,
        proxy_query::ProxyQuery,
    },
    make_new_session_cookie,
};

pub async fn random_post(
    State(state): State<AppState>,
    Query(query): Query<RandomPostRequest>,
) -> Result<Json<HttpDanbooruPost>, AilyError> {
    let post = state
        .database
        .get_random_post(*query.include_nsfw())
        .await
        .inspect_err(|e| error!("Error in random_post: {}", e))?;

    Ok(Json(HttpDanbooruPost::from(&post)))
}

pub async fn start_game(
    State(state): State<AppState>,
    cookies: CookieJar,
) -> Result<(CookieJar, Json<HttpDanbooruPost>), AilyError> {
    let id = Uuid::new_v4().to_string();

    let post = state
        .database
        .get_random_post(false)
        .await
        .inspect_err(|e| error!("Error while getting random post in start: {}", e))?;

    let session = Session::new(vec![], *post.id(), get_new_session_exptime());

    state.sessions.write().await.insert(id.clone(), session);

    let cookies = cookies.add(make_new_session_cookie(id));

    Ok((cookies, Json(HttpDanbooruPost::from(&post))))
}

pub async fn guess(
    State(state): State<AppState>,
    Query(query): Query<GuessQuery>,
    logged_session: LoggedSession,
) -> Result<(CookieJar, Json<(bool, HttpDanbooruPost)>), AilyError> {
    let session_id = &logged_session.1;

    let post_id = {
        let sessions = state.sessions.read().await;
        let session = sessions
            .get(session_id)
            .ok_or(AilyError::InvalidSession)
            .inspect_err(|e| warn!("Got request with invalid session in guess: {}", e))?;
        *session.current()
    };

    let post = state
        .database
        .get_post_by_id(post_id)
        .await
        .inspect_err(|e| error!("Error while getting post in guess: {}", e))?;
    let guessed_correctly = *post.is_ai_generated() == *query.is_ai_generated();
    let next = state
        .database
        .get_random_post(*query.include_nsfw())
        .await
        .inspect_err(|e| error!("Error while getting next post in guess: {}", e))?;

    {
        let mut sessions = state.sessions.write().await;

        let session = sessions
            .get_mut(session_id)
            .ok_or(AilyError::InvalidSession)?;
        session.posts_mut().push((*post.id(), guessed_correctly));
        session.set_current(*next.id());
    }

    Ok((
        logged_session.0,
        Json((guessed_correctly, HttpDanbooruPost::from(&next))),
    ))
}

pub async fn current_session_post(
    State(state): State<AppState>,
    logged_session: LoggedSession,
) -> Result<(CookieJar, Json<HttpDanbooruPost>), AilyError> {
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

    Ok((logged_session.0, Json(HttpDanbooruPost::from(&post))))
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
        .map_err(|_| AilyError::ProxyError)
        .inspect_err(|e| error!("Error while sending request in proxy: {}", e))?;

    if !resp.status().is_success() {
        return Err(AilyError::ProxyError)
            .inspect_err(|e| error!("Remote server returned error in proxy: {}", e));
    }

    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    let bytes = resp.bytes().await.map_err(|_| AilyError::Unknown)?;

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

pub async fn current_session(
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
