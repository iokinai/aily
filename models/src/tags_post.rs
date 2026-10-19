use trl::Trl;

use crate::{Artwork, Tag};

#[derive(Trl, Clone)]
#[getters]
#[constructor]
pub struct TagsPost {
    id: i32,
    danbooru_id: i32,
    path: String,
    is_nsfw: bool,
    tags: Vec<Tag>,
}

impl From<(Artwork, Vec<Tag>)> for TagsPost {
    fn from(value: (Artwork, Vec<Tag>)) -> Self {
        TagsPost {
            id: *value.0.id(),
            danbooru_id: *value.0.danbooru_id(),
            path: value.0.path().clone(),
            is_nsfw: *value.0.is_nsfw(),
            tags: value.1,
        }
    }
}
