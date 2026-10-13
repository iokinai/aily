use diesel::{Selectable, associations::Identifiable, deserialize::Queryable};

use crate::schema::artworks;

#[derive(Queryable, Selectable, Identifiable, Debug, Clone)]
#[diesel(table_name = artworks)]
pub struct DbArtwork {
    pub id: i32,
    pub danbooru_id: i32,
    pub path: String,
    pub is_nsfw: bool,
}
