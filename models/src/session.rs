use trl::Trl;

use crate::{AilyPost, Artwork};

#[derive(Trl)]
#[getters(option)]
#[constructor]
pub struct Session {
    prev: Vec<(AilyPost, bool)>,
    tags_prev: Vec<(Artwork, Vec<(String, bool)>, Option<bool>)>,
    current: Option<Artwork>,
    tags_current: Option<Artwork>,
}
