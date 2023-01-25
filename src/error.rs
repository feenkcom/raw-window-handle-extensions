use crate::{RawDisplayHandleType, RawWindowHandleType};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct UnsupportedWindowHandleError {
    handle_type: RawWindowHandleType,
}

impl UnsupportedWindowHandleError {
    pub fn new(handle_type: RawWindowHandleType) -> Self {
        Self { handle_type }
    }
}

impl Display for UnsupportedWindowHandleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unsupported window handle type: {:?}", self.handle_type)
    }
}

impl Error for UnsupportedWindowHandleError {}

#[derive(Debug)]
pub struct UnsupportedDisplayHandleError {
    handle_type: RawDisplayHandleType,
}

impl UnsupportedDisplayHandleError {
    pub fn new(handle_type: RawDisplayHandleType) -> Self {
        Self { handle_type }
    }
}

impl Display for UnsupportedDisplayHandleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unsupported display handle type: {:?}", self.handle_type)
    }
}

impl Error for UnsupportedDisplayHandleError {}
