use trl::Trl;

#[derive(Trl, Clone)]
#[getters]
#[constructor]
pub struct Artwork {
    id: i32,
    danbooru_id: i32,
    path: String,
    is_nsfw: bool,
}
