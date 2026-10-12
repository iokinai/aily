use aily_models::AilyPost;
use serde::Serialize;
use trl::Trl;

#[derive(Trl, Serialize)]
#[getters]
#[constructor]
pub struct HttpAilyPost {
    id: i32,
    danbooru_id: i32,
    path: String,
    is_ai_generated: bool,
}

impl From<&AilyPost> for HttpAilyPost {
    fn from(value: &AilyPost) -> Self {
        HttpAilyPost::new(
            *value.id(),
            *value.danbooru_id(),
            value.path().clone(),
            *value.is_ai_generated(),
        )
    }
}
