use axum::{
    Json,
    extract::{Query, State},
};
use axum_extra::extract::CookieJar;

use crate::{
    errors::AilyError,
    game::tags::SessionEnded,
    http::{
        AppState, ArtworkRequest, ArtworkToPostRequest, HttpArtwork, HttpSession, HttpSessionEnded,
        HttpSurrenderResult, HttpTag, HttpTagGuessResult, HttpTagsPost, LoggedSession,
        SurrenderRequest, TagsGuessRequest,
    },
};

pub async fn load_tags_artwork(
    State(state): State<AppState>,
    logged_session: LoggedSession,
    Query(query): Query<ArtworkRequest>,
) -> Result<(CookieJar, Json<HttpArtwork>), AilyError> {
    let session_id = logged_session.1;

    let artwork = state
        .tags_game_controller
        .load_tags_artwork(*query.include_nsfw())
        .await?;

    let mut sessions = state.sessions.write().await;
    let session = sessions
        .get_mut(&session_id)
        .ok_or(AilyError::InvalidSession)?;

    session.set_tags_current(Some(*artwork.id()));

    Ok((logged_session.0, Json(HttpArtwork::from(&artwork))))
}

pub async fn tags_make_guess(
    State(state): State<AppState>,
    logged_session: LoggedSession,
    Json(req): Json<TagsGuessRequest>,
) -> Result<Json<(HttpTagGuessResult, HttpSession)>, AilyError> {
    let session_id = logged_session.1;

    let correct = state
        .tags_game_controller
        .make_guess(&session_id, req.tag())
        .await?;

    let ended = state
        .tags_game_controller
        .update_session_after_guess(&session_id, req.tag(), correct, *req.include_nsfw())
        .await?;

    let http_session_ended = match ended {
        SessionEnded::No => HttpSessionEnded::No,
        SessionEnded::Yes(won, artwork) => HttpSessionEnded::Yes {
            won,
            next: HttpArtwork::from(&artwork),
        },
    };

    Ok(Json((
        HttpTagGuessResult::new(correct, http_session_ended),
        HttpSession::from(
            &state
                .global_game_controller
                .game_session_to_global_session(
                    &state
                        .global_game_controller
                        .get_session_by_id(&session_id)
                        .await?,
                )
                .await?,
        ),
    )))
}

pub async fn get_tags_list(State(state): State<AppState>) -> Result<Json<Vec<HttpTag>>, AilyError> {
    Ok(Json(
        state
            .tags_game_controller
            .get_tags_list()
            .await?
            .iter()
            .map(|t| HttpTag::from(t))
            .collect(),
    ))
}

pub async fn artwork_to_tags_post(
    State(state): State<AppState>,
    Query(query): Query<ArtworkToPostRequest>,
) -> Result<Json<HttpTagsPost>, AilyError> {
    let result = state
        .tags_game_controller
        .get_tags_post_by_artwork_id(*query.id())
        .await?;

    Ok(Json(HttpTagsPost::from(&result)))
}

pub async fn tags_surrender(
    State(state): State<AppState>,
    logged_session: LoggedSession,
    Json(request): Json<SurrenderRequest>,
) -> Result<(CookieJar, Json<(HttpSurrenderResult, HttpSession)>), AilyError> {
    let session_id = logged_session.1;

    let surrender_result = state
        .tags_game_controller
        .surrender(&session_id, *request.include_nsfw())
        .await?;

    let sessions = state.sessions.read().await;
    let session = sessions.get(&session_id).ok_or(AilyError::InvalidSession)?;

    Ok((
        logged_session.0,
        Json((
            HttpSurrenderResult::from(&surrender_result),
            HttpSession::from(
                &state
                    .global_game_controller
                    .game_session_to_global_session(session)
                    .await?,
            ),
        )),
    ))
}
