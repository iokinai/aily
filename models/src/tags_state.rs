use serde::Serialize;

use crate::TagsFinish;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TagsState {
    Going,
    Finished(TagsFinish),
}

impl TagsState {
    pub fn is_finished(&self) -> bool {
        match self {
            TagsState::Going => false,
            TagsState::Finished(_) => true,
        }
    }
}
