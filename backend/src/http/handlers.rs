use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
use log::{error, warn};
use uuid::Uuid;

use crate::{
    errors::AilyError,
    game::Session,
    get_new_session_lifetime,
    http::{
        AppState, GuessQuery, HttpDanbooruPost, LoggedSession, RandomPostRequest,
        proxy_query::ProxyQuery,
    },
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
) -> Result<Json<(String, HttpDanbooruPost)>, AilyError> {
    let id = Uuid::new_v4().to_string();

    let post = state
        .database
        .get_random_post(false)
        .await
        .inspect_err(|e| error!("Error while getting random post in start: {}", e))?;

    let session = Session::new(vec![], *post.id(), get_new_session_lifetime());

    state.sessions.write().await.insert(id.clone(), session);

    Ok(Json((id, HttpDanbooruPost::from(&post))))
}

pub async fn guess(
    State(state): State<AppState>,
    Query(query): Query<GuessQuery>,
    logged_session: LoggedSession,
) -> Result<Json<(bool, HttpDanbooruPost)>, AilyError> {
    let session_id = &logged_session.0;

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
        session.posts().push((*post.id(), guessed_correctly));
        session.set_current(*next.id());
    }

    Ok(Json((guessed_correctly, HttpDanbooruPost::from(&next))))
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
