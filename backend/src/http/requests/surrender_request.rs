use serde::Deserialize;
use trl::Trl;

#[derive(Deserialize, Trl)]
#[serde(rename_all = "snake_case")]
#[getters]
pub struct SurrenderRequest {
    include_nsfw: bool,
}
