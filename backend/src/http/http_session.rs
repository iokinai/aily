use aily_models::GameSession;
use serde::Serialize;

use crate::http::HttpDanbooruPost;

#[derive(Serialize)]
pub struct HttpSession {
    prev: Vec<(HttpDanbooruPost, bool)>,
    current: HttpDanbooruPost,
}

impl From<&GameSession> for HttpSession {
    #[inline]
    fn from(value: &GameSession) -> Self {
        let prev = value
            .prev()
            .iter()
            .map(|p| (HttpDanbooruPost::from(&p.0), p.1))
            .collect();

        let current = HttpDanbooruPost::from(value.current());

        HttpSession { prev, current }
    }
}
