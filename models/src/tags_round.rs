use serde::Serialize;
use trl::Trl;

#[derive(Trl, Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[getters]
#[constructor]
pub struct TagsRound {
    tag: String,
    guessed_correctly: bool,
}
