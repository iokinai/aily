use std::time::Instant;

use trl::Trl;

// All posts vectors should ALWAYS contain the `artwork_id`, not the entity id
#[derive(Trl, Clone, Debug)]
#[constructor]
pub struct GameSession {
    #[get]
    #[get(mut ref, name = aily_posts_mut)]
    aily_posts: Vec<(i32, bool)>,
    #[get]
    #[get(mut ref, name = tag_posts_mut)]
    tag_posts: Vec<(i32, Vec<(String, bool)>, Option<bool>)>,
    #[get(option)]
    #[set(option)]
    aily_current: Option<i32>,
    #[get(option)]
    #[set(option)]
    tags_current: Option<i32>,
    #[get]
    #[set]
    expires_at: Instant,
}

// #[cfg(test)]
// mod tests {
//     use std::time::Instant;
//
//     use time::Duration;
//
//     use crate::game::GameSession;
//
//     #[test]
//     fn test_new_session() {
//         let posts = vec![(1, true), (2, false)];
//         let tag_posts = vec![];
//         let current = 3;
//         let tags_current = 4;
//         let expires_at = Instant::now() + Duration::seconds(10);
//
//         let session = GameSession::new(
//             posts.clone(),
//             tag_posts.clone(),
//             Some(current),
//             Some(tags_current),
//             expires_at,
//         );
//
//         assert_eq!(*session.aily_posts(), posts);
//         assert_eq!(*session.tag_posts(), tag_posts);
//         assert_eq!(session.aily_current().cloned(), Some(current));
//         assert_eq!(session.tags_current().cloned(), Some(tags_current));
//         assert_eq!(*session.expires_at(), expires_at);
//     }
// }
