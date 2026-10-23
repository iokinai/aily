use std::time::Instant;

use aily_models::TagsRound;
use trl::Trl;

use crate::game::{GameAilySession, GameTagsSession, game_tags_state::GameTagsState};

// All posts vectors should ALWAYS contain the `artwork_id`, not the entity id
#[derive(Trl, Clone, Debug)]
#[constructor]
pub struct GameSession {
    #[get]
    #[get(mut ref, name = aily_posts_mut)]
    aily_posts: Vec<GameAilySession>,
    #[get]
    #[get(mut ref, name = tag_posts_mut)]
    tag_posts: Vec<GameTagsSession>,
    #[get]
    #[set]
    expires_at: Instant,
    #[get(option)]
    #[set(option)]
    aily_current: Option<i32>,
    #[get(option)]
    #[set(option)]
    tags_current: Option<i32>,
}

impl GameSession {
    pub fn append_aily_session(&mut self, artwork_id: i32, guessed_correctly: bool) {
        self.aily_posts
            .push(GameAilySession::new(artwork_id, guessed_correctly));
    }

    pub fn append_tags_session(
        &mut self,
        artwork_id: i32,
        rounds: Vec<TagsRound>,
        state: GameTagsState,
    ) {
        self.tag_posts
            .push(GameTagsSession::new(artwork_id, rounds, state));
    }

    pub fn start_new_tags_session(&mut self, artwork_id: i32) {
        self.tag_posts.push(GameTagsSession::new(
            artwork_id,
            vec![],
            GameTagsState::Going,
        ));
        self.tags_current = Some(artwork_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        game::{GameSession, game_tags_state::GameTagsState},
        get_new_session_exptime,
    };

    #[test]
    fn test_append_aily_session() {
        let mut session = GameSession::new(vec![], vec![], get_new_session_exptime(), None, None);

        session.append_aily_session(2, false);

        assert_eq!(session.aily_posts().len(), 1);
        assert_eq!(*session.aily_posts().get(0).unwrap().artwork_id(), 2);
        assert_eq!(
            *session.aily_posts().get(0).unwrap().guessed_correctly(),
            false
        );
    }

    #[test]
    fn test_append_tags_session() {
        let mut session = GameSession::new(vec![], vec![], get_new_session_exptime(), None, None);

        session.append_tags_session(2, vec![], GameTagsState::Going);

        assert_eq!(session.tag_posts().len(), 1);
        assert_eq!(*session.tag_posts().get(0).unwrap().artwork_id(), 2);
        assert_eq!(session.tag_posts().get(0).unwrap().rounds().len(), 0);

        let is_state_going =
            if let GameTagsState::Going = session.tag_posts().get(0).unwrap().state() {
                true
            } else {
                false
            };

        assert!(is_state_going)
    }

    #[test]
    fn test_start_new_tags_session() {
        let mut session = GameSession::new(vec![], vec![], get_new_session_exptime(), None, None);

        session.start_new_tags_session(2);

        assert_eq!(session.tag_posts().len(), 1);
        assert_eq!(*session.tag_posts().get(0).unwrap().artwork_id(), 2);
        assert_eq!(*session.tags_current().unwrap(), 2);
        assert_eq!(session.tag_posts().get(0).unwrap().rounds().len(), 0);

        let is_state_going =
            if let GameTagsState::Going = session.tag_posts().get(0).unwrap().state() {
                true
            } else {
                false
            };

        assert!(is_state_going)
    }
}
