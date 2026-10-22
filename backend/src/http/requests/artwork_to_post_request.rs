use serde::Deserialize;
use trl::Trl;

#[derive(Trl, Deserialize)]
#[getters]
pub struct ArtworkToPostRequest {
    id: i32,
}
