use serde::Deserialize;
use trl::Trl;

#[derive(Deserialize, Trl)]
#[getters]
pub struct ArtworkRequest {
    include_nsfw: bool,
}
