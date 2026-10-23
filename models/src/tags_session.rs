use trl::Trl;

use crate::{Artwork, TagsRound, TagsState};

#[derive(Trl)]
#[getters]
#[constructor]
pub struct TagsSession {
    artwork: Artwork,
    rounds: Vec<TagsRound>,
    state: TagsState,
}
