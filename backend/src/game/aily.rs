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
            session.aily_posts_mut().push((c, guessed_correctly));
        }

        session.set_aily_current(Some(*new_post.id()));
        Ok(())
    }

    pub async fn post_by_artwork_id(&self, artwork_id: i32) -> Result<AilyPost, AilyError> {
        Ok(self.database.get_post_by_artwork_id(artwork_id).await?)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::sync::Arc;
//     use std::time::{Duration, Instant};
//     use tokio::sync::RwLock;
//
//     #[test]
//     fn test_has_user_guessed_correctly() {
//         let ai_post = AilyPost::new(1, 1, 1, true, true, "".to_string());
//         let real_post = AilyPost::new(1, 1, 1, false, true, "".to_string());
//
//         assert!(has_user_guessed_correctly(&ai_post, true));
//         assert!(!has_user_guessed_correctly(&ai_post, false));
//
//         assert!(has_user_guessed_correctly(&real_post, false));
//         assert!(!has_user_guessed_correctly(&real_post, true));
//     }
//
//     #[tokio::test]
//     async fn test_update_session_after_guess() {
//         let sessions = Arc::new(RwLock::new(HashMap::new()));
//         let session_id = "test-uuid-123".to_string();
//
//         let initial_session = GameSession::new(
//             vec![],
//             vec![],
//             Some(10),
//             None,
//             Instant::now() + Duration::from_secs(3600),
//         );
//         sessions
//             .write()
//             .await
//             .insert(session_id.clone(), initial_session);
//
//         let new_post = Artwork::new(20, 1, "".to_string(), true);
//
//         let result =
//             update_session_after_guess(sessions.clone(), &session_id, &new_post, true).await;
//
//         assert!(result.is_ok());
//
//         let sessions_read = sessions.read().await;
//         let session = sessions_read.get(&session_id).unwrap();
//
//         assert_eq!(session.aily_current().cloned(), Some(20));
//
//         assert_eq!(session.aily_posts().len(), 1);
//         assert_eq!(session.aily_posts()[0], (10, true));
//     }
// }
