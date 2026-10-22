use std::{collections::HashMap, sync::Arc};

use aily_models::{AilyPost, Artwork};
use log::{error, warn};
use tokio::sync::RwLock;

use crate::{
    database::Database,
    errors::{AilyError, NotFound},
    game::GameSession,
};

#[derive(Clone)]
pub struct AilyGameController {
    sessions: Arc<RwLock<HashMap<String, GameSession>>>,
    database: Arc<dyn Database + Send + Sync>,
}

impl AilyGameController {
    pub fn new(
        sessions: Arc<RwLock<HashMap<String, GameSession>>>,
        database: Arc<dyn Database + Send + Sync>,
    ) -> AilyGameController {
        AilyGameController { sessions, database }
    }

    pub async fn make_guess(
        &self,
        session_id: &String,
        is_ai_generated: bool,
    ) -> Result<bool, AilyError> {
        let post_id = self
            .sessions
            .read()
            .await
            .get(session_id)
            .ok_or(AilyError::InvalidSession)
            .inspect_err(|_| warn!("Got request with invalid session in guess"))?
            .aily_current()
            .cloned();

        if let None = post_id {
            return Err(AilyError::NotFound(NotFound::Artwork));
        }

        let post_id = post_id.unwrap();

        let post = self
            .database
            .get_post_by_artwork_id(post_id)
            .await
            .inspect_err(|e| error!("Error while getting post in guess: {}", e))?;

        Ok(self.has_user_guessed_correctly(&post, is_ai_generated))
    }

    pub fn has_user_guessed_correctly(&self, post: &AilyPost, guess_is_ai: bool) -> bool {
        *post.is_ai_generated() == guess_is_ai
    }

    pub async fn load_artwork(&self, include_nsfw: bool) -> Result<Artwork, AilyError> {
        Ok(self
            .database
            .get_random_artwork(include_nsfw)
            .await
            .inspect_err(|e| error!("Error while getting next post in guess: {}", e))?)
    }

    pub async fn update_session_after_guess(
        &self,
        session_id: &String,
        new_post: &Artwork,
        guessed_correctly: bool,
    ) -> Result<(), AilyError> {
        let mut sessions = self.sessions.write().await;

        let session = sessions
            .get_mut(session_id)
            .ok_or(AilyError::InvalidSession)?;
        let current = session.aily_current().cloned();

        if let Some(c) = current {
            session.append_aily_session(c, guessed_correctly);
        }

        session.set_aily_current(Some(*new_post.id()));
        Ok(())
    }

    pub async fn post_by_artwork_id(&self, artwork_id: i32) -> Result<AilyPost, AilyError> {
        Ok(self.database.get_post_by_artwork_id(artwork_id).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_new_session_exptime, mocks::db_mock::MockTestDatabase};
    use std::collections::HashMap;
    use tokio::sync::RwLock;

    async fn setup_sessions(
        session_id: &str,
        current_artwork_id: i32,
    ) -> Arc<RwLock<HashMap<String, GameSession>>> {
        let sessions = Arc::new(RwLock::new(HashMap::new()));

        let mut session = GameSession::new(vec![], vec![], get_new_session_exptime(), None, None);
        session.set_aily_current(Some(current_artwork_id));

        sessions
            .write()
            .await
            .insert(session_id.to_string(), session);

        sessions
    }

    #[tokio::test]
    async fn test_make_guess_success() {
        let session_id = "test_session".to_string();
        let artwork_id = 42;

        let sessions = setup_sessions(&session_id, artwork_id).await;

        let mut mock_db = MockTestDatabase::new();

        mock_db
            .expect_get_post_by_artwork_id()
            .with(mockall::predicate::eq(artwork_id))
            .times(1)
            .returning(|id| {
                let post = AilyPost::new(id, 1, 1, true, true, String::from(""));
                Ok(post)
            });

        let controller = AilyGameController::new(sessions, Arc::new(mock_db));

        let result = controller.make_guess(&session_id, true).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_make_guess_invalid_session() {
        let mock_db = MockTestDatabase::new();
        let sessions = Arc::new(RwLock::new(HashMap::new()));
        let controller = AilyGameController::new(sessions, Arc::new(mock_db));

        let result = controller
            .make_guess(&"non_existent".to_string(), true)
            .await;

        assert!(matches!(result, Err(AilyError::InvalidSession)));
    }

    #[tokio::test]
    async fn test_load_artwork() {
        let mut mock_db = MockTestDatabase::new();

        mock_db
            .expect_get_random_artwork()
            .with(mockall::predicate::eq(false))
            .times(1)
            .returning(|_| Ok(Artwork::new(1, 1, "".to_string(), false)));

        let controller =
            AilyGameController::new(Arc::new(RwLock::new(HashMap::new())), Arc::new(mock_db));

        let result = controller.load_artwork(false).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_has_user_guessed_correctly() {
        let controller = AilyGameController::new(
            Arc::new(RwLock::new(HashMap::new())),
            Arc::new(MockTestDatabase::new()),
        );

        let post = AilyPost::new(1, 1, 1, true, false, String::from(""));
        let res = controller.has_user_guessed_correctly(&post, true);
        assert!(res);

        let post = AilyPost::new(1, 1, 1, false, false, String::from(""));
        let res = controller.has_user_guessed_correctly(&post, true);
        assert!(!res);
    }
}
