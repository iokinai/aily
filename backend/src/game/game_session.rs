use std::time::Instant;

use trl::Trl;

#[derive(Trl, Clone)]
#[constructor]
pub struct GameSession {
    #[get]
    #[get(mut ref, name = posts_mut)]
    posts: Vec<(i32, bool)>,
    #[set]
    #[get]
    current: i32,
    #[get]
    #[set]
    expires_at: Instant,
}
