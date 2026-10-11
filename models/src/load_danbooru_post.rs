use serde::Deserialize;
use trl::Trl;

#[derive(Deserialize, Trl)]
#[getters]
pub struct LoadDanbooruPost {
    id: u64,
    tag_string: String,
    file_url: Option<String>,
    rating: String,
}
