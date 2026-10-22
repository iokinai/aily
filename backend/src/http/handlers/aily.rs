use axum::{
    Json,
    extract::{Query, State},
};
use axum_extra::extract::CookieJar;

use crate::{
    errors::AilyError,
    http::{
        AppState, ArtworkRequest, ArtworkToPostRequest, GuessRequest, HttpAilyPost, HttpArtwork,
        HttpSession, LoggedSession,
    },
};

pub async fn make_guess(
    State(state): State<AppState>,
    logged_session: LoggedSession,
    Json(query): Json<GuessRequest>,
) -> Result<(CookieJar, Json<(bool, HttpArtwork, HttpSession)>), AilyError> {
    let session_id = &logged_session.1;

    let correct = state
        .aily_game_controller
        .make_guess(session_id, *query.is_ai_generated())
        .await?;

    let next_artwork = state
        .aily_game_controller
        .load_artwork(*query.include_nsfw())
        .await?;

    state
        .aily_game_controller
        .update_session_after_guess(session_id, &next_artwork, correct)
        .await?;

    Ok((
        logged_session.0,
        Json((
            correct,
            HttpArtwork::from(&next_artwork),
            HttpSession::from(
                &state
                    .global_game_controller
                    .game_session_to_global_session(
                        &state
                            .global_game_controller
                            .get_session_by_id(session_id)
                            .await?,
                    )
                    .await?,
            ),
        )),
    ))
}

pub async fn artwork_to_post(
    State(state): State<AppState>,
    Query(query): Query<ArtworkToPostRequest>,
) -> Result<Json<HttpAilyPost>, AilyError> {
    let result = state
        .aily_game_controller
        .post_by_artwork_id(*query.id())
        .await?;

    Ok(Json(HttpAilyPost::from(&result)))
}

pub async fn load_artwork(
    State(state): State<AppState>,
    Query(query): Query<ArtworkRequest>,
    logged_session: LoggedSession,
) -> Result<Json<HttpArtwork>, AilyError> {
    let session_id = logged_session.1;

    let artwork = state
        .aily_game_controller
        .load_artwork(*query.include_nsfw())
        .await?;

    let mut sessions = state.sessions.write().await;
    let session = sessions
        .get_mut(&session_id)
        .ok_or(AilyError::InvalidSession)?;
    session.set_aily_current(Some(*artwork.id()));

    Ok(Json(HttpArtwork::from(&artwork)))
}
