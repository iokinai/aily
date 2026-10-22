use aily_models::{TagsFinish, TagsRound};
use trl::Trl;

use crate::game::game_tags_state::GameTagsState;

#[derive(Trl, Clone, Debug)]
#[getters]
#[constructor]
pub struct GameTagsSession {
    artwork_id: i32,
    rounds: Vec<TagsRound>,
    state: GameTagsState,
}

impl GameTagsSession {
    pub fn append_round(&mut self, tag: String, correct: bool) {
        self.rounds.push(TagsRound::new(tag, correct));
    }

    pub fn end_session(&mut self, artwork_id: i32, reason: TagsFinish) {
        self.state = GameTagsState::Finished {
            post: artwork_id,
            finish: reason,
        };
    }
}
