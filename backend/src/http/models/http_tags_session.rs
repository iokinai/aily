use aily_models::{TagsRound, TagsSession, TagsState};
use serde::Serialize;
use trl::Trl;

use crate::http::HttpArtwork;

#[derive(Trl, Serialize)]
#[serde(rename_all = "snake_case")]
#[constructor]
pub struct HttpTagsSession {
    artwork: HttpArtwork,
    rounds: Vec<TagsRound>,
    state: TagsState,
}

impl From<&TagsSession> for HttpTagsSession {
    fn from(value: &TagsSession) -> Self {
        HttpTagsSession {
            artwork: HttpArtwork::from(value.artwork()),
            rounds: value.rounds().clone(),
            state: value.state().clone(),
        }
    }
}
