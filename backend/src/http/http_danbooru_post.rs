use aily_models::GameDanbooruPost;
use serde::Serialize;
use trl::Trl;

#[derive(Serialize, Trl)]
#[getters]
pub struct HttpDanbooruPost {
    id: i32,
    danbooru_id: Option<i32>,
    path: String,
    is_ai_generated: bool,
}

impl From<&GameDanbooruPost> for HttpDanbooruPost {
    #[inline]
    fn from(value: &GameDanbooruPost) -> Self {
        HttpDanbooruPost {
            id: *value.id(),
            danbooru_id: *value.danbooru_id(),
            path: value.path().clone(),
            is_ai_generated: *value.is_ai_generated(),
        }
    }
}
