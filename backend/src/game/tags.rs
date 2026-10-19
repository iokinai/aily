use std::{collections::HashMap, sync::Arc};

use aily_models::{Artwork, SurrenderResult, Tag, TagsPost};
use tokio::sync::RwLock;

use crate::{
    database::Database,
    errors::{AilyError, NotFound, TagAlreadyUsed},
    game::GameSession,
};

pub enum ShouldSessionEnd {
    No,
    Yes(bool),
}

pub enum SessionEnded {
    No,
    Yes(bool, Artwork),
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
            for (guessed_tag, _) in round.1.iter() {
                if *tag == *guessed_tag {
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
        current_guess_session: &mut (i32, Vec<(String, bool)>, Option<bool>),
    ) -> ShouldSessionEnd {
        let correct_guesses = current_guess_session.1.iter().filter(|v| v.1).count();

        if correct_guesses >= self.correct_guesses_until_win as usize {
            return ShouldSessionEnd::Yes(true);
        }

        if current_guess_session.1.len() < self.max_session_length as usize {
            return ShouldSessionEnd::No;
        }

        ShouldSessionEnd::Yes(correct_guesses >= self.correct_guesses_until_win as usize)
    }

    pub async fn update_session_after_guess(
        &self,
        session_id: &String,
        tag: &String,
        guessed: bool,
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

        if session.tag_posts().is_empty() || session.tag_posts_mut().last().unwrap().0 != current_id
        {
            session.tag_posts_mut().push((current_id, vec![], None));
        }

        let current_guess_session = session.tag_posts_mut().last_mut().unwrap();

        current_guess_session.1.push((tag.clone(), guessed));

        let should_end = self.should_session_end_with_result(current_guess_session);

        if let ShouldSessionEnd::Yes(e) = should_end {
            current_guess_session.2 = Some(e);
            session
                .tag_posts_mut()
                .push((*next_artwork.id(), vec![], None));
            session.set_tags_current(Some(*next_artwork.id()));
            return Ok(SessionEnded::Yes(e, next_artwork));
        }

        Ok(SessionEnded::No)
    }

    pub async fn get_tags_list(&self) -> Result<Vec<Tag>, AilyError> {
        self.database.get_tags_list().await
    }

    pub async fn get_tags_post_by_artwork_id(&self, id: i32) -> Result<TagsPost, AilyError> {
        self.database.get_tags_post_by_artwork_id(id).await
    }

    pub async fn surrender(
        &self,
        session_id: &String,
        include_nsfw: bool,
    ) -> Result<SurrenderResult, AilyError> {
        let next = self.load_tags_artwork(include_nsfw).await?;

        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(session_id)
            .ok_or(AilyError::InvalidSession)?;

        if let None = session.tags_current() {
            return Err(AilyError::NotFound(NotFound::Artwork));
        }

        let current_id = *session.tags_current().unwrap();

        let current_post = self.get_tags_post_by_artwork_id(current_id).await?;

        if session.tag_posts().is_empty() || session.tag_posts_mut().last().unwrap().0 != current_id
        {
            session.tag_posts_mut().push((current_id, vec![], None));
        }

        let current_guess_session = session.tag_posts_mut().last_mut().unwrap();

        current_guess_session.2 = Some(false);

        session.tag_posts_mut().push((*next.id(), vec![], None));

        Ok(SurrenderResult::new(current_post, next))
    }
}
