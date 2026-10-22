use serde::Serialize;

use crate::{TagsFinish, TagsPost};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TagsState {
    Going,
    Finished { post: TagsPost, finish: TagsFinish },
}
