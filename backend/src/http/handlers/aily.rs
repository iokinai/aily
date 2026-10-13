use axum::{Json, extract::State};
use axum_extra::extract::CookieJar;
use log::{error, warn};

use crate::{
    errors::AilyError,
    game::{
        aily::{has_user_guessed_correctly, update_session_after_guess},
        global::get_session_by_id,
    },
    http::{AppState, GuessQuery, HttpAilyPost, LoggedSession},
};

pub async fn make_guess(
    State(state): State<AppState>,
    logged_session: LoggedSession,
    Json(query): Json<GuessQuery>,
) -> Result<(CookieJar, Json<(bool, HttpAilyPost)>), AilyError> {
    let session_id = &logged_session.1;

    let post_id = *get_session_by_id(state.sessions.clone(), session_id)
        .await
        .inspect_err(|e| warn!("Got request with invalid session in guess: {}", e))?
        .current();

    let post = state
        .database
        .get_post_by_id(post_id)
        .await
        .inspect_err(|e| error!("Error while getting post in guess: {}", e))?;

    let guessed_correctly = has_user_guessed_correctly(&post, *query.is_ai_generated());

    let next = state
        .database
        .get_random_post(*query.include_nsfw())
        .await
        .inspect_err(|e| error!("Error while getting next post in guess: {}", e))?;

    update_session_after_guess(state.sessions.clone(), session_id, &next, guessed_correctly)
        .await?;

    Ok((
        logged_session.0,
        Json((guessed_correctly, HttpAilyPost::from(&next))),
    ))
}
