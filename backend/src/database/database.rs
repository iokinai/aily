use aily_models::{GameDanbooruPost, GameSession};
use async_trait::async_trait;

use crate::{errors::AilyError, game::Session};

#[async_trait]
pub trait Database {
    async fn get_random_post(&self, include_nsfw: bool) -> Result<GameDanbooruPost, AilyError>;
    async fn get_post_by_id(&self, id: i32) -> Result<GameDanbooruPost, AilyError>;
    async fn get_session(&self, sess: &Session) -> Result<GameSession, AilyError>;
}
