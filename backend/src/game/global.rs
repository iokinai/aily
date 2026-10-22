use std::{collections::HashMap, sync::Arc};

use aily_models::{
    AilyPost, AilySession, Artwork, Session, TagsPost, TagsRound, TagsSession, TagsState,
};
use tokio::sync::RwLock;

use crate::{
    database::Database,
    errors::AilyError,
    game::{GameAilySession, GameSession, GameTagsSession, game_tags_state::GameTagsState},
    get_new_session_exptime,
};

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
        let session = GameSession::new(vec![], vec![], get_new_session_exptime(), None, None);

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
        aily_posts: &[GameAilySession],
        current_id: Option<i32>,
    ) -> Result<(Vec<AilySession>, Option<Artwork>), AilyError> {
        let mut ids: Vec<i32> = aily_posts.iter().map(|p| *p.artwork_id()).collect();
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
        for entry in aily_posts {
            if let Some(post) = posts_map.get(&entry.artwork_id()) {
                prev.push(AilySession::new(post.clone(), *entry.guessed_correctly()));
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
        tag_posts: &[GameTagsSession],
        current_id: Option<i32>,
    ) -> Result<(Vec<TagsSession>, Option<Artwork>), AilyError> {
        let mut ids: Vec<i32> = tag_posts.iter().map(|p| *p.artwork_id()).collect();

        for entry in tag_posts {
            if let GameTagsState::Finished { post, .. } = entry.state() {
                ids.push(*post);
            }
        }

        if let Some(c) = current_id {
            ids.push(c);
        }

        ids.sort();
        ids.dedup();

        if ids.is_empty() {
            return Ok((Vec::new(), None));
        }

        let tags_posts = db.get_tags_posts_by_artwork_ids(&ids).await?;
        let posts_map: HashMap<i32, TagsPost> =
            tags_posts.into_iter().map(|tp| (*tp.id(), tp)).collect();

        let mut tags_prev = Vec::with_capacity(tag_posts.len());
        for entry in tag_posts {
            if let Some(tags_post) = posts_map.get(&entry.artwork_id()) {
                let artwork = Artwork::new(
                    *tags_post.id(),
                    *tags_post.danbooru_id(),
                    tags_post.path().clone(),
                    *tags_post.is_nsfw(),
                );

                let rounds: Vec<TagsRound> = entry
                    .rounds()
                    .iter()
                    .map(|r| TagsRound::new(r.tag().clone(), *r.guessed_correctly()))
                    .collect();

                let state = match entry.state() {
                    GameTagsState::Going => TagsState::Going,
                    GameTagsState::Finished { post, finish } => {
                        let finished_post =
                            posts_map.get(post).cloned().ok_or(AilyError::Unknown)?;

                        TagsState::Finished {
                            post: finished_post,
                            finish: finish.clone(),
                        }
                    }
                };

                tags_prev.push(TagsSession::new(artwork, rounds, state));
            }
        }

        let tags_current = current_id.and_then(|c| posts_map.get(&c)).map(|tp| {
            Artwork::new(
                *tp.id(),
                *tp.danbooru_id(),
                tp.path().clone(),
                *tp.is_nsfw(),
            )
        });

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::db_mock::MockTestDatabase;
    use std::collections::HashMap;
    use tokio::sync::RwLock;

    fn setup_controller(
        mock_db: MockTestDatabase,
    ) -> (
        GlobalGameController,
        Arc<RwLock<HashMap<String, GameSession>>>,
    ) {
        let sessions = Arc::new(RwLock::new(HashMap::new()));
        let controller = GlobalGameController::new(sessions.clone(), Arc::new(mock_db));
        (controller, sessions)
    }

    #[tokio::test]
    async fn test_make_and_save_new_session() {
        let mock_db = MockTestDatabase::new();
        let (controller, sessions) = setup_controller(mock_db);

        let session_id = "test_uuid_123".to_string();
        controller.make_and_save_new_session(&session_id).await;

        let sessions_guard = sessions.read().await;
        assert!(sessions_guard.contains_key(&session_id));
    }

    #[tokio::test]
    async fn test_get_session_by_id_success() {
        let mock_db = MockTestDatabase::new();
        let (controller, sessions) = setup_controller(mock_db);

        let session_id = "my_session".to_string();
        let new_session = GameSession::new(vec![], vec![], get_new_session_exptime(), None, None);
        sessions
            .write()
            .await
            .insert(session_id.clone(), new_session);

        let result = controller.get_session_by_id(&session_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_session_by_id_invalid() {
        let mock_db = MockTestDatabase::new();
        let (controller, _) = setup_controller(mock_db);

        let result = controller
            .get_session_by_id(&"non_existent".to_string())
            .await;
        assert!(matches!(result, Err(AilyError::InvalidSession)));
    }
}
