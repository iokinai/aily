use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TagsLoseReason {
    NotGuessed,
    Surrender,
}
