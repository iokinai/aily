use aily_models::SurrenderResult;
use serde::Serialize;

use crate::http::{HttpArtwork, HttpTagsPost};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpSurrenderResult {
    post: HttpTagsPost,
    next: HttpArtwork,
}

impl From<&SurrenderResult> for HttpSurrenderResult {
    fn from(value: &SurrenderResult) -> Self {
        HttpSurrenderResult {
            post: HttpTagsPost::from(value.post()),
            next: HttpArtwork::from(value.next()),
        }
    }
}
