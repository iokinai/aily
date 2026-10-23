use aily_models::Tag;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpTag {
    id: i32,
    tag_name: String,
}

impl From<&Tag> for HttpTag {
    fn from(value: &Tag) -> Self {
        HttpTag {
            id: *value.id(),
            tag_name: value.tag_name().clone(),
        }
    }
}
