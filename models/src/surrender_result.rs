use trl::Trl;

use crate::{Artwork, TagsPost};

// TODO: Unused
#[derive(Trl)]
#[getters]
#[constructor]
pub struct SurrenderResult {
    post: TagsPost,
    next: Artwork,
}
