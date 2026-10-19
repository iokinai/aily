use trl::Trl;

use crate::{Artwork, TagsPost};

#[derive(Trl)]
#[getters]
#[constructor]
pub struct SurrenderResult {
    post: TagsPost,
    next: Artwork,
}
