use aily_models::Session;
use serde::Serialize;

use crate::http::{HttpAilyPost, HttpArtwork};

#[derive(Serialize)]
pub struct HttpSession {
    prev: Vec<(HttpAilyPost, bool)>,
    tags_prev: Vec<(HttpArtwork, Vec<(String, bool)>, Option<bool>)>,
    current: Option<HttpArtwork>,
    tags_current: Option<HttpArtwork>,
}

impl From<&Session> for HttpSession {
    #[inline]
    fn from(value: &Session) -> Self {
        let prev = value
            .prev()
            .iter()
            .map(|p| (HttpAilyPost::from(&p.0), p.1))
            .collect();

        let tags_prev = value
            .tags_prev()
            .iter()
            .map(|p| (HttpArtwork::from(&p.0), p.1.clone(), p.2))
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
