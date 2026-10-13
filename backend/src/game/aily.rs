use std::{collections::HashMap, sync::Arc};

use aily_models::AilyPost;
use tokio::sync::RwLock;

use crate::{errors::AilyError, game::GameSession};

pub fn has_user_guessed_correctly(post: &AilyPost, guess_is_ai: bool) -> bool {
    *post.is_ai_generated() == guess_is_ai
}

pub async fn update_session_after_guess(
    sessions: Arc<RwLock<HashMap<String, GameSession>>>,
    session_id: &String,
    new_post: &AilyPost,
    guessed_correctly: bool,
) -> Result<(), AilyError> {
    let mut sessions = sessions.write().await;

    let session = sessions
        .get_mut(session_id)
        .ok_or(AilyError::InvalidSession)?;
    let current = *session.current();
    session.posts_mut().push((current, guessed_correctly));
    session.set_current(*new_post.id());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::{Duration, Instant};
    use tokio::sync::RwLock;

    #[test]
    fn test_has_user_guessed_correctly() {
        let ai_post = AilyPost::new(1, 1, 1, true, true, "".to_string());
        let real_post = AilyPost::new(1, 1, 1, false, true, "".to_string());

        assert!(has_user_guessed_correctly(&ai_post, true));
        assert!(!has_user_guessed_correctly(&ai_post, false));

        assert!(has_user_guessed_correctly(&real_post, false));
        assert!(!has_user_guessed_correctly(&real_post, true));
    }

    #[tokio::test]
    async fn test_update_session_after_guess() {
        let sessions = Arc::new(RwLock::new(HashMap::new()));
        let session_id = "test-uuid-123".to_string();

        let initial_session =
            GameSession::new_with_current(10, Instant::now() + Duration::from_secs(3600));
        sessions
            .write()
            .await
            .insert(session_id.clone(), initial_session);

        let new_post = AilyPost::new(20, 1, 1, true, true, "".to_string());

        let result =
            update_session_after_guess(sessions.clone(), &session_id, &new_post, true).await;

        assert!(result.is_ok());

        let sessions_read = sessions.read().await;
        let session = sessions_read.get(&session_id).unwrap();

        assert_eq!(*session.current(), 20);

        assert_eq!(session.posts().len(), 1);
        assert_eq!(session.posts()[0], (10, true));
    }
}
