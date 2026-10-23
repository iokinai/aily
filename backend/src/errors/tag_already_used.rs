use core::fmt;

use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug, Error)]
#[serde(rename_all = "snake_case")]
pub struct TagAlreadyUsed {
    name: String,
}

impl fmt::Display for TagAlreadyUsed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tag {} already used in this round", &self.name)
    }
}

impl TagAlreadyUsed {
    pub fn new(name: &String) -> TagAlreadyUsed {
        TagAlreadyUsed { name: name.clone() }
    }
}
