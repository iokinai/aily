use aily_models::TagsFinish;

#[derive(Clone, Debug)]
pub enum GameTagsState {
    Going,
    // Post must be an artwork_id
    Finished { post: i32, finish: TagsFinish },
}

impl GameTagsState {
    pub fn is_finished(&self) -> bool {
        match self {
            GameTagsState::Going => false,
            GameTagsState::Finished { post: _, finish: _ } => true,
        }
    }
}
