use trl::Trl;

#[derive(Trl, Clone)]
#[getters]
#[constructor]
pub struct AilyPost {
    id: i32,
    artwork_id: i32,
    danbooru_id: i32,
    is_ai_generated: bool,
    is_nsfw: bool,
    path: String,
}
