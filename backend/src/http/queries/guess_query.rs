use serde::Deserialize;
use trl::Trl;

#[derive(Deserialize, Trl)]
#[getters]
pub struct GuessQuery {
    include_nsfw: bool,
    is_ai_generated: bool,
}
