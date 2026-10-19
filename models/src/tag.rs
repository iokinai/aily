use trl::Trl;

#[derive(Trl, Clone)]
#[getters]
#[constructor]
pub struct Tag {
    id: i32,
    tag_name: String,
}
