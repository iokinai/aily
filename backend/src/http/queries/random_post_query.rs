use serde::Deserialize;
use trl::Trl;

#[derive(Deserialize, Trl)]
#[constructor]
#[getters]
pub struct RandomPostQuery {
    include_nsfw: bool,
}
