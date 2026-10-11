use trl::Trl;

use crate::GameDanbooruPost;

#[derive(Trl)]
#[getters]
#[constructor]
pub struct GameSession {
    prev: Vec<(GameDanbooruPost, bool)>,
    current: GameDanbooruPost,
}
