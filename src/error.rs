use crate::{RawDisplayHandleType, RawWindowHandleType};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum RawError {
    UnsupportedWindowHandle(RawWindowHandleType),
    UnsupportedDisplayHandle(RawDisplayHandleType),
    NullPointer,
}

impl Display for RawError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RawError::UnsupportedWindowHandle(handle_type) => {
                write!(f, "Unsupported window handle type: {:?}", handle_type)
            }
            RawError::UnsupportedDisplayHandle(handle_type) => {
                write!(f, "Unsupported display handle type: {:?}", handle_type)
            }
            RawError::NullPointer => {
                write!(f, "Nil pointer")
            }
        }
    }
}

impl Error for RawError {}
