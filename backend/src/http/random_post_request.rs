use serde::Deserialize;
use trl::Trl;

#[derive(Deserialize, Trl)]
#[constructor]
#[getters]
pub struct RandomPostRequest {
    include_nsfw: bool,
}
