use std::time::Instant;

use trl::Trl;

#[derive(Trl)]
#[constructor]
pub struct Session {
    #[get(mut ref)]
    posts: Vec<(i32, bool)>,
    #[set]
    #[get]
    current: i32,
    #[get]
    #[set]
    expires_at: Instant,
}
