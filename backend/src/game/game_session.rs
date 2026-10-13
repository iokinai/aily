use std::time::Instant;

use trl::Trl;

#[derive(Trl, Clone, Debug)]
#[constructor]
pub struct GameSession {
    #[get]
    #[get(mut ref, name = posts_mut)]
    posts: Vec<(i32, bool)>,
    #[set]
    #[get]
    current: i32,
    #[get]
    #[set]
    expires_at: Instant,
}

impl GameSession {
    pub fn new_with_current(current: i32, expires_at: Instant) -> GameSession {
        GameSession {
            posts: vec![],
            current,
            expires_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use time::Duration;

    use crate::game::GameSession;

    #[test]
    fn test_new_session() {
        let posts = vec![(1, true), (2, false)];
        let current = 3;
        let expires_at = Instant::now() + Duration::seconds(10);

        let session = GameSession::new(posts.clone(), current, expires_at);

        assert_eq!(*session.posts(), posts);
        assert_eq!(*session.current(), current);
        assert_eq!(*session.expires_at(), expires_at);
    }

    #[test]
    fn test_new_session_with_current() {
        let current = 1;
        let expires_at = Instant::now() + Duration::seconds(10);

        let session = GameSession::new_with_current(current, expires_at);

        assert_eq!(*session.posts(), vec![]);
        assert_eq!(*session.current(), current);
        assert_eq!(*session.expires_at(), expires_at);
    }
}
