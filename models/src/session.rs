use trl::Trl;

use crate::AilyPost;

#[derive(Trl)]
#[getters]
#[constructor]
pub struct Session {
    prev: Vec<(AilyPost, bool)>,
    current: AilyPost,
}
