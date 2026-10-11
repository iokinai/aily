use trl::Trl;

#[derive(Trl)]
#[getters]
#[constructor]
pub struct GameDanbooruPost {
    id: i32,
    danbooru_id: Option<i32>,
    is_nsfw: bool,
    is_ai_generated: bool,
    is_ai_assisted: bool,
    path: String,
}
