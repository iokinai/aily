use std::{collections::HashMap, sync::Arc};

use aily_models::{AilyPost, Artwork, Session};
use tokio::sync::RwLock;

use crate::{database::Database, errors::AilyError, game::GameSession, get_new_session_exptime};

#[derive(Clone)]
pub struct GlobalGameController {
    sessions: Arc<RwLock<HashMap<String, GameSession>>>,
    database: Arc<dyn Database + Send + Sync>,
}

impl GlobalGameController {
    pub fn new(
        sessions: Arc<RwLock<HashMap<String, GameSession>>>,
        database: Arc<dyn Database + Send + Sync>,
    ) -> GlobalGameController {
        GlobalGameController { sessions, database }
    }

    pub async fn make_and_save_new_session(&self, id: &String) {
        let session = GameSession::new(vec![], vec![], None, None, get_new_session_exptime());

        self.sessions.write().await.insert(id.clone(), session);
    }

    pub async fn get_session_by_id(&self, session_id: &String) -> Result<GameSession, AilyError> {
        let sessions = self.sessions.read().await;
        Ok(sessions
            .get(session_id)
            .ok_or(AilyError::InvalidSession)?
            .clone())
    }

    async fn fetch_aily_history(
        &self,
        db: &Arc<dyn Database + Send + Sync>,
        aily_posts: &[(i32, bool)],
        current_id: Option<i32>,
    ) -> Result<(Vec<(AilyPost, bool)>, Option<Artwork>), AilyError> {
        let mut ids: Vec<i32> = aily_posts.iter().map(|(id, _)| *id).collect();
        if let Some(c) = current_id {
            ids.push(c);
        }

        if ids.is_empty() {
            return Ok((Vec::new(), None));
        }

        let posts = db.get_posts_by_artwork_ids(&ids).await?;
        let posts_map: HashMap<i32, AilyPost> =
            posts.into_iter().map(|post| (*post.id(), post)).collect();

        let mut prev = Vec::with_capacity(aily_posts.len());
        for (id, flag) in aily_posts {
            if let Some(post) = posts_map.get(id) {
                prev.push((post.clone(), *flag));
            }
        }

        let current = if let Some(c) = current_id {
            let artworks = db.get_artworks_by_ids(&[c]).await?;
            artworks.into_iter().next()
        } else {
            None
        };

        Ok((prev, current))
    }

    async fn fetch_tags_history(
        &self,
        db: &Arc<dyn Database + Send + Sync>,
        tag_posts: &[(i32, Vec<(String, bool)>, Option<bool>)],
        current_id: Option<i32>,
    ) -> Result<
        (
            Vec<(Artwork, Vec<(String, bool)>, Option<bool>)>,
            Option<Artwork>,
        ),
        AilyError,
    > {
        let mut ids: Vec<i32> = tag_posts.iter().map(|(id, _, _)| *id).collect();
        if let Some(c) = current_id {
            ids.push(c);
        }

        if ids.is_empty() {
            return Ok((Vec::new(), None));
        }

        let artworks = db.get_artworks_by_ids(&ids).await?;
        let artworks_map: HashMap<i32, Artwork> =
            artworks.into_iter().map(|art| (*art.id(), art)).collect();

        let mut tags_prev = Vec::with_capacity(tag_posts.len());
        for (id, tags, ended) in tag_posts {
            if let Some(artwork) = artworks_map.get(id) {
                tags_prev.push((artwork.clone(), tags.clone(), (*ended).clone()));
            }
        }

        let tags_current = current_id.and_then(|c| artworks_map.get(&c)).cloned();

        Ok((tags_prev, tags_current))
    }

    pub async fn game_session_to_global_session(
        &self,
        game_session: &GameSession,
    ) -> Result<Session, AilyError> {
        let (prev, current) = self
            .fetch_aily_history(
                &self.database,
                game_session.aily_posts(),
                game_session.aily_current().copied(),
            )
            .await?;

        let (tags_prev, tags_current) = self
            .fetch_tags_history(
                &self.database,
                game_session.tag_posts(),
                game_session.tags_current().copied(),
            )
            .await?;

        Ok(Session::new(prev, tags_prev, current, tags_current))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::sync::Arc;
//     use std::time::{Duration, Instant};
//     use tokio::sync::RwLock;
//
//     #[tokio::test]
//     async fn test_get_session_by_id() {
//         let sessions = Arc::new(RwLock::new(HashMap::new()));
//         let session_id = "test-session-uuid".to_string();
//
//         let original_session = GameSession::new(
//             vec![],
//             vec![],
//             Some(42),
//             None,
//             Instant::now() + Duration::from_secs(3600),
//         );
//         sessions
//             .write()
//             .await
//             .insert(session_id.clone(), original_session);
//
//         let result = get_session_by_id(sessions.clone(), &session_id).await;
//         assert!(result.is_ok());
//         let fetched_session = result.unwrap();
//         assert_eq!(fetched_session.aily_current().cloned(), Some(42));
//         assert_eq!(fetched_session.tags_current().cloned(), None);
//
//         let missing_id = "non-existent-id".to_string();
//         let missing_result = get_session_by_id(sessions.clone(), &missing_id).await;
//         assert!(missing_result.is_err());
//         assert!(matches!(
//             missing_result.unwrap_err(),
//             AilyError::InvalidSession
//         ));
//     }
// }
