use aily_models::Artwork;
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

impl From<&DbArtwork> for Artwork {
    fn from(value: &DbArtwork) -> Self {
        Artwork::new(
            value.id,
            value.danbooru_id,
            value.path.clone(),
            value.is_nsfw,
        )
    }
}
