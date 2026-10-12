use axum::{
    Json,
    extract::{Query, State},
};
use axum_extra::extract::CookieJar;
use log::{error, warn};

use crate::{
    errors::AilyError,
    http::{AppState, GuessQuery, HttpAilyPost, LoggedSession, RandomPostQuery},
};

pub async fn random_post(
    State(state): State<AppState>,
    Query(query): Query<RandomPostQuery>,
) -> Result<Json<HttpAilyPost>, AilyError> {
    let post = state
        .database
        .get_random_post(*query.include_nsfw())
        .await
        .inspect_err(|e| error!("Error in random_post: {}", e))?;

    Ok(Json(HttpAilyPost::from(&post)))
}

pub async fn make_guess(
    State(state): State<AppState>,
    logged_session: LoggedSession,
    Json(query): Json<GuessQuery>,
) -> Result<(CookieJar, Json<(bool, HttpAilyPost)>), AilyError> {
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
        Json((guessed_correctly, HttpAilyPost::from(&next))),
    ))
}
