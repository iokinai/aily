use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NotFound {
    #[error("artwork")]
    Artwork,
    #[error("tag")]
    Tag,
}
