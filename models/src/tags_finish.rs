use serde::Serialize;

use crate::TagsLoseReason;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TagsFinish {
    Win,
    Lose(TagsLoseReason),
}
