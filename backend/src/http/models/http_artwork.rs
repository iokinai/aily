use aily_models::Artwork;
use serde::Serialize;
use trl::Trl;

#[derive(Serialize, Trl)]
#[getters]
pub struct HttpArtwork {
    id: i32,
    danbooru_id: i32,
    path: String,
    is_nsfw: bool,
}

impl From<&Artwork> for HttpArtwork {
    fn from(value: &Artwork) -> Self {
        HttpArtwork {
            id: *value.id(),
            danbooru_id: *value.danbooru_id(),
            path: value.path().clone(),
            is_nsfw: *value.is_nsfw(),
        }
    }
}
