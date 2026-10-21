use std::{collections::HashMap, sync::Arc};

use aily_models::{Artwork, Tag, TagsFinish, TagsLoseReason, TagsPost, TagsState};
use tokio::sync::RwLock;

use crate::{
    database::Database,
    errors::{AilyError, NotFound, TagAlreadyUsed},
    game::{GameSession, GameTagsSession},
};

pub enum ShouldSessionEnd {
    No,
    Yes(TagsFinish),
}

pub enum SessionEnded {
    No,
    Yes(bool, Artwork),
}

pub enum GuessType {
    Guess { tag: String, correct: bool },
    Surrender,
}

#[derive(Clone)]
pub struct TagsGameController {
    sessions: Arc<RwLock<HashMap<String, GameSession>>>,
    database: Arc<dyn Database + Send + Sync>,
    max_session_length: u16,
    correct_guesses_until_win: u16,
}

impl TagsGameController {
    pub fn new(
        sessions: Arc<RwLock<HashMap<String, GameSession>>>,
        database: Arc<dyn Database + Send + Sync>,
        max_session_length: u16,
        correct_guesses_until_win: u16,
    ) -> TagsGameController {
        TagsGameController {
            sessions,
            database,
            max_session_length,
            correct_guesses_until_win,
        }
    }

    pub async fn load_tags_artwork(&self, include_nsfw: bool) -> Result<Artwork, AilyError> {
        self.database.get_aily_tags_artwork(include_nsfw).await
    }

    pub async fn make_guess(&self, session_id: &String, tag: &String) -> Result<bool, AilyError> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(session_id).ok_or(AilyError::InvalidSession)?;

        if let Some(round) = session.tag_posts().last() {
            for round in round.rounds().iter() {
                if *tag == *round.tag() {
                    return Err(AilyError::TagAlreadyUsed(TagAlreadyUsed::new(tag)));
                }
            }
        }

        let artwork_id = session.tags_current().cloned();

        if let None = artwork_id {
            return Err(AilyError::NotFound(NotFound::Artwork));
        }

        let artwork_id = artwork_id.unwrap();

        self.database.make_tags_guess(tag, artwork_id).await
    }

    pub fn should_session_end_with_result(
        &self,
        current_guess_session: &GameTagsSession,
    ) -> ShouldSessionEnd {
        let correct_guesses = current_guess_session
            .rounds()
            .iter()
            .filter(|v| *v.guessed_correctly())
            .count();

        if correct_guesses >= self.correct_guesses_until_win as usize {
            return ShouldSessionEnd::Yes(TagsFinish::Win);
        }

        if current_guess_session.rounds().len() < self.max_session_length as usize {
            return ShouldSessionEnd::No;
        }

        ShouldSessionEnd::Yes(TagsFinish::Lose(TagsLoseReason::NotGuessed))
    }

    pub async fn update_session_after_guess(
        &self,
        session_id: &String,
        guess_type: GuessType,
        include_nsfw: bool,
    ) -> Result<SessionEnded, AilyError> {
        let next_artwork = self.database.get_aily_tags_artwork(include_nsfw).await?;

        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(session_id)
            .ok_or(AilyError::InvalidSession)?;

        let Some(current) = session.tags_current() else {
            session.set_tags_current(Some(*next_artwork.id()));
            return Ok(SessionEnded::No);
        };
        let current_id = *current;

        if session.tag_posts().is_empty()
            || session.tag_posts().last().unwrap().state().is_finished()
        {
            session.append_tags_session(current_id, vec![], TagsState::Going);
        }

        let current_session = session.tag_posts_mut().last_mut().unwrap();

        match guess_type {
            GuessType::Guess { tag, correct } => {
                current_session.append_round(tag, correct);
            }
            GuessType::Surrender => {
                current_session.end_session(TagsFinish::Lose(TagsLoseReason::Surrender));
                session.start_new_tags_session(*next_artwork.id());
                return Ok(SessionEnded::Yes(false, next_artwork));
            }
        }

        let should_end = self.should_session_end_with_result(current_session);

        if let ShouldSessionEnd::Yes(e) = should_end {
            current_session.end_session(e.clone());
            session.start_new_tags_session(*next_artwork.id());

            let is_win = if let TagsFinish::Win = e { true } else { false };

            return Ok(SessionEnded::Yes(is_win, next_artwork));
        }

        Ok(SessionEnded::No)
    }

    pub async fn get_tags_list(&self) -> Result<Vec<Tag>, AilyError> {
        self.database.get_tags_list().await
    }

    pub async fn get_tags_post_by_artwork_id(&self, id: i32) -> Result<TagsPost, AilyError> {
        self.database.get_tags_post_by_artwork_id(id).await
    }

    pub async fn surrender(&self, session_id: &String) -> Result<TagsPost, AilyError> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(session_id)
            .ok_or(AilyError::InvalidSession)?;

        if let None = session.tags_current() {
            return Err(AilyError::NotFound(NotFound::Artwork));
        }

        let current_id = *session.tags_current().unwrap();

        let current_post = self.get_tags_post_by_artwork_id(current_id).await?;

        Ok(current_post)
    }
}
