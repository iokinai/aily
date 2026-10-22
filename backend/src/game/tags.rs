use std::{collections::HashMap, sync::Arc};

use aily_models::{Artwork, Tag, TagsFinish, TagsLoseReason, TagsPost};
use tokio::sync::RwLock;

use crate::{
    database::Database,
    errors::{AilyError, NotFound, TagAlreadyUsed},
    game::{GameSession, GameTagsSession, game_tags_state::GameTagsState},
};

#[derive(Debug)]
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
            session.append_tags_session(current_id, vec![], GameTagsState::Going);
        }

        let current_session = session.tag_posts_mut().last_mut().unwrap();

        match guess_type {
            GuessType::Guess { tag, correct } => {
                current_session.append_round(tag, correct);
            }
            GuessType::Surrender => {
                current_session
                    .end_session(current_id, TagsFinish::Lose(TagsLoseReason::Surrender));
                session.start_new_tags_session(*next_artwork.id());
                return Ok(SessionEnded::Yes(false, next_artwork));
            }
        }

        let should_end = self.should_session_end_with_result(current_session);

        if let ShouldSessionEnd::Yes(e) = should_end {
            current_session.end_session(current_id, e.clone());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::db_mock::MockTestDatabase;
    use aily_models::TagsRound;
    use std::collections::HashMap;
    use tokio::sync::RwLock;

    fn setup_tags_controller(
        mock_db: MockTestDatabase,
        max_len: u16,
        win_count: u16,
    ) -> (
        TagsGameController,
        Arc<RwLock<HashMap<String, GameSession>>>,
    ) {
        let sessions = Arc::new(RwLock::new(HashMap::new()));
        let controller =
            TagsGameController::new(sessions.clone(), Arc::new(mock_db), max_len, win_count);
        (controller, sessions)
    }

    #[tokio::test]
    async fn test_make_guess_invalid_session() {
        let mock_db = MockTestDatabase::new();
        let (controller, _) = setup_tags_controller(mock_db, 5, 3);

        let result = controller
            .make_guess(&"wrong_session".to_string(), &"cat".to_string())
            .await;
        assert!(matches!(result, Err(AilyError::InvalidSession)));
    }

    #[tokio::test]
    async fn test_load_tags_artwork() {
        let mut mock_db = MockTestDatabase::new();
        mock_db
            .expect_get_aily_tags_artwork()
            .with(mockall::predicate::eq(false))
            .times(1)
            .returning(|_| Ok(Artwork::new(1, 1, String::from(""), false)));

        let (controller, _) = setup_tags_controller(mock_db, 5, 3);

        let result = controller.load_tags_artwork(false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_tags_list() {
        let mut mock_db = MockTestDatabase::new();
        mock_db
            .expect_get_tags_list()
            .times(1)
            .returning(|| Ok(vec![]));

        let (controller, _) = setup_tags_controller(mock_db, 5, 3);

        let result = controller.get_tags_list().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_should_session_end_with_win() {
        let mock_db = MockTestDatabase::new();
        let (controller, _) = setup_tags_controller(mock_db, 5, 2);

        let game_session = GameTagsSession::new(
            1,
            vec![
                TagsRound::new(String::from(""), true),
                TagsRound::new(String::from(""), true),
            ],
            GameTagsState::Going,
        );

        let decision = controller.should_session_end_with_result(&game_session);

        let ended_with_win = if let ShouldSessionEnd::Yes(TagsFinish::Win) = decision {
            true
        } else {
            false
        };

        assert!(ended_with_win);
    }

    #[test]
    fn test_should_session_end_with_lose() {
        let mock_db = MockTestDatabase::new();
        let (controller, _) = setup_tags_controller(mock_db, 5, 2);

        let game_session = GameTagsSession::new(
            1,
            vec![
                TagsRound::new(String::from(""), false),
                TagsRound::new(String::from(""), false),
                TagsRound::new(String::from(""), false),
                TagsRound::new(String::from(""), false),
                TagsRound::new(String::from(""), false),
            ],
            GameTagsState::Going,
        );

        let decision = controller.should_session_end_with_result(&game_session);

        let ended_with_win =
            if let ShouldSessionEnd::Yes(TagsFinish::Lose(TagsLoseReason::NotGuessed)) = decision {
                true
            } else {
                false
            };

        assert!(ended_with_win);
    }

    #[tokio::test]
    async fn test_surrender_invalid_session() {
        let mock_db = MockTestDatabase::new();
        let (controller, _) = setup_tags_controller(mock_db, 5, 3);

        let result = controller.surrender(&"non_existent".to_string()).await;
        assert!(matches!(result, Err(AilyError::InvalidSession)));
    }
}
