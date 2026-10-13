use aily_models::Session;
use serde::Serialize;

use crate::http::HttpAilyPost;

#[derive(Serialize)]
pub struct HttpSession {
    prev: Vec<(HttpAilyPost, bool)>,
    current: HttpAilyPost,
}

impl From<&Session> for HttpSession {
    #[inline]
    fn from(value: &Session) -> Self {
        let prev = value
            .prev()
            .iter()
            .map(|p| (HttpAilyPost::from(&p.0), p.1))
            .collect();

        let current = HttpAilyPost::from(value.current());

        HttpSession { prev, current }
    }
}
