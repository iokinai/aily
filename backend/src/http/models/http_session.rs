use aily_models::Session;
use serde::Serialize;

use crate::http::{HttpAilySession, HttpArtwork, HttpTagsSession};

#[derive(Serialize)]
pub struct HttpSession {
    prev: Vec<HttpAilySession>,
    tags_prev: Vec<HttpTagsSession>,
    current: Option<HttpArtwork>,
    tags_current: Option<HttpArtwork>,
}

impl From<&Session> for HttpSession {
    #[inline]
    fn from(value: &Session) -> Self {
        let prev = value
            .prev()
            .iter()
            .map(|p| HttpAilySession::from(p))
            .collect();

        let tags_prev = value
            .tags_prev()
            .iter()
            .map(|p| HttpTagsSession::from(p))
            .collect();

        let current = if let Some(c) = value.current() {
            Some(HttpArtwork::from(c))
        } else {
            None
        };

        let tags_current = if let Some(c) = value.tags_current() {
            Some(HttpArtwork::from(c))
        } else {
            None
        };

        HttpSession {
            prev,
            tags_prev,
            current,
            tags_current,
        }
    }
}
