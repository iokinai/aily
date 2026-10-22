use trl::Trl;

use crate::{Artwork, TagsSession, aily_session::AilySession};

#[derive(Trl)]
#[getters(option)]
#[constructor]
pub struct Session {
    prev: Vec<AilySession>,
    tags_prev: Vec<TagsSession>,
    current: Option<Artwork>,
    tags_current: Option<Artwork>,
}
