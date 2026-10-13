use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::{errors::AilyError, game::GameSession};

pub async fn get_session_by_id(
    sessions: Arc<RwLock<HashMap<String, GameSession>>>,
    session_id: &String,
) -> Result<GameSession, AilyError> {
    let sessions = sessions.read().await;
    Ok(sessions
        .get(session_id)
        .ok_or(AilyError::InvalidSession)?
        .clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::{Duration, Instant};
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_get_session_by_id() {
        let sessions = Arc::new(RwLock::new(HashMap::new()));
        let session_id = "test-session-uuid".to_string();

        let original_session =
            GameSession::new_with_current(42, Instant::now() + Duration::from_secs(3600));
        sessions
            .write()
            .await
            .insert(session_id.clone(), original_session);

        let result = get_session_by_id(sessions.clone(), &session_id).await;
        assert!(result.is_ok());
        let fetched_session = result.unwrap();
        assert_eq!(*fetched_session.current(), 42);

        let missing_id = "non-existent-id".to_string();
        let missing_result = get_session_by_id(sessions.clone(), &missing_id).await;
        assert!(missing_result.is_err());
        assert!(matches!(
            missing_result.unwrap_err(),
            AilyError::InvalidSession
        ));
    }
}
