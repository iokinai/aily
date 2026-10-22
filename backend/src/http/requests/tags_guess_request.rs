use serde::Deserialize;
use trl::Trl;

#[derive(Trl, Deserialize)]
#[getters]
pub struct TagsGuessRequest {
    include_nsfw: bool,
    tag: String,
}
