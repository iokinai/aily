use diesel::{Selectable, deserialize::Queryable};

use crate::database::db_artwork::DbArtwork;
use crate::schema::aily_posts;

use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone)]
#[diesel(belongs_to(DbArtwork, foreign_key = artwork_id))]
#[diesel(table_name = aily_posts)]
pub struct DbAilyPost {
    pub id: i32,
    pub artwork_id: i32,
    pub is_ai_generated: bool,
}
