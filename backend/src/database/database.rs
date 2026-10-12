use aily_models::{AilyPost, Session};
use async_trait::async_trait;

use crate::{errors::AilyError, game::GameSession};

#[async_trait]
pub trait Database {
    async fn get_random_post(&self, include_nsfw: bool) -> Result<AilyPost, AilyError>;
    async fn get_post_by_id(&self, id: i32) -> Result<AilyPost, AilyError>;
    async fn get_session(&self, sess: &GameSession) -> Result<Session, AilyError>;
}
