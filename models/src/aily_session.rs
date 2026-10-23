use trl::Trl;

use crate::AilyPost;

#[derive(Trl)]
#[getters]
#[constructor]
pub struct AilySession {
    post: AilyPost,
    correct: bool,
}
