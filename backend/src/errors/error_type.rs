use serde_repr::Serialize_repr;

#[derive(Serialize_repr)]
#[repr(u32)]
pub enum ErrorType {
    NotFound = 0,
    InvalidSession = 1,
    ProxyError = 2,
    Unknown = 3,
    TagAlreadyUsed = 4,
}
