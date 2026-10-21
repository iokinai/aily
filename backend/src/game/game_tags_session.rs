use aily_models::{TagsFinish, TagsRound, TagsState};
use trl::Trl;

#[derive(Trl, Clone, Debug)]
#[getters]
#[constructor]
pub struct GameTagsSession {
    artwork_id: i32,
    rounds: Vec<TagsRound>,
    state: TagsState,
}

impl GameTagsSession {
    pub fn append_round(&mut self, tag: String, correct: bool) {
        self.rounds.push(TagsRound::new(tag, correct));
    }

    pub fn end_session(&mut self, reason: TagsFinish) {
        self.state = TagsState::Finished(reason);
    }
}
