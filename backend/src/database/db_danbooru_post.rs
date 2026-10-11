use aily_models::GameDanbooruPost;
use diesel::deserialize::{Queryable, QueryableByName};
use trl::Trl;

use crate::schema::Arts;

#[derive(Queryable, QueryableByName, Trl)]
#[diesel(table_name = Arts)]
#[getters]
pub struct DbDanbooruPost {
    id: i32,
    danbooru_id: Option<i32>,
    is_nsfw: bool,
    is_ai_generated: bool,
    is_ai_assisted: bool,
    path: String,
}

impl From<&DbDanbooruPost> for GameDanbooruPost {
    #[inline]
    fn from(value: &DbDanbooruPost) -> Self {
        GameDanbooruPost::new(
            value.id,
            value.danbooru_id,
            value.is_nsfw,
            value.is_ai_generated,
            value.is_ai_assisted,
            value.path.clone(),
        )
    }
}
