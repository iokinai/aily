use aily_models::AilySession;
use serde::Serialize;
use trl::Trl;

use crate::http::HttpAilyPost;

#[derive(Serialize, Trl)]
#[getters]
#[serde(rename_all = "snake_case")]
pub struct HttpAilySession {
    post: HttpAilyPost,
    correct: bool,
}

impl From<&AilySession> for HttpAilySession {
    fn from(value: &AilySession) -> Self {
        HttpAilySession {
            post: HttpAilyPost::from(value.post()),
            correct: *value.correct(),
        }
    }
}
