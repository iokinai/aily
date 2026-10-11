use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct LoadError;

impl Error for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while loading")
    }
}
