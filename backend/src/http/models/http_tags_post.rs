use aily_models::TagsPost;
use serde::Serialize;

use crate::http::HttpTag;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpTagsPost {
    id: i32,
    danbooru_id: i32,
    path: String,
    is_nsfw: bool,
    tags: Vec<HttpTag>,
}

impl From<&TagsPost> for HttpTagsPost {
    fn from(value: &TagsPost) -> Self {
        HttpTagsPost {
            id: *value.id(),
            danbooru_id: *value.danbooru_id(),
            path: value.path().clone(),
            is_nsfw: *value.is_nsfw(),
            tags: value.tags().iter().map(|t| HttpTag::from(t)).collect(),
        }
    }
}
