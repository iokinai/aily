use aily_models::{AilyPost, Artwork, Tag, TagsPost};
use async_trait::async_trait;

use crate::errors::AilyError;

#[async_trait]
pub trait Database {
    async fn get_random_artwork(&self, include_nsfw: bool) -> Result<Artwork, AilyError>;
    async fn get_aily_tags_artwork(&self, include_nsfw: bool) -> Result<Artwork, AilyError>;
    async fn get_artworks_by_ids(&self, ids: &[i32]) -> Result<Vec<Artwork>, AilyError>;
    async fn get_posts_by_artwork_ids(&self, ids: &[i32]) -> Result<Vec<AilyPost>, AilyError>;
    async fn get_tags_posts_by_artwork_ids(&self, ids: &[i32]) -> Result<Vec<TagsPost>, AilyError>;
    async fn get_post_by_artwork_id(&self, id: i32) -> Result<AilyPost, AilyError>;
    async fn make_tags_guess(&self, tag: &String, artwork_id: i32) -> Result<bool, AilyError>;
    async fn get_tags_list(&self) -> Result<Vec<Tag>, AilyError>;
    async fn get_tags_post_by_artwork_id(&self, id: i32) -> Result<TagsPost, AilyError>;
}
