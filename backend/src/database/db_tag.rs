use aily_models::Tag;
use diesel::{Selectable, deserialize::Queryable};

use crate::schema::tags;

#[derive(Queryable, Selectable)]
#[diesel(table_name = tags)]
pub struct DbTag {
    id: i32,
    tag_name: String,
}

impl From<&DbTag> for Tag {
    fn from(value: &DbTag) -> Self {
        Tag::new(value.id, value.tag_name.clone())
    }
}
